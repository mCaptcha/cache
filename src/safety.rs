/*
 * Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use std::time::Duration;

use redis_module::key::RedisKeyWritable;
use redis_module::native_types::RedisType;
use redis_module::raw::KeyType;
use redis_module::NotifyEvent;
use redis_module::{raw, Context};
use serde::{Deserialize, Serialize};

use crate::bucket::Bucket;
use crate::errors::*;
use crate::mcaptcha::MCaptcha;
use crate::utils::*;

const MCAPTCHA_SAFETY_VERSION: i32 = 0;

#[derive(Serialize, Deserialize)]
pub struct MCaptchaSafety;

impl MCaptchaSafety {
    pub fn on_delete(ctx: &Context, _event_type: NotifyEvent, _event: &str, key_name: &str) {
        if !is_mcaptcha_safety(key_name) {
            return;
        }

        let mcaptcha_name = get_mcaptcha_from_safety(key_name);
        if mcaptcha_name.is_none() {
            return;
        }
        let mcaptcha_name = mcaptcha_name.unwrap();
        let mcaptcha = ctx.open_key(mcaptcha_name);
        if mcaptcha.key_type() == KeyType::Empty {
            ctx.log_warning(&format!("mcaptcha {} is empty", mcaptcha_name));
            return;
        }

        let mcaptcha_val = MCaptcha::get_mcaptcha(&mcaptcha);
        if mcaptcha_val.is_err() {
            ctx.log_warning(&format!(
                "error occured while trying to access mcaptcha {}. error {} is empty",
                mcaptcha_name,
                mcaptcha_val.err().unwrap()
            ));
            return;
        }
        let mcaptcha_val = mcaptcha_val.unwrap();
        if mcaptcha_val.is_none() {
            ctx.log_warning(&format!(
                "error occured while trying to access mcaptcha {}. is none",
                mcaptcha_name,
            ));
            return;
        }
        let mcaptcha_val = mcaptcha_val.unwrap();
        let duration = mcaptcha_val.get_duration();
        let visitors = mcaptcha_val.get_visitors();

        if Self::new(ctx, duration, mcaptcha_name).is_err() {
            ctx.log_warning(&format!(
                "error occured while creating safety for mcaptcha {}.",
                mcaptcha_name,
            ));
        };
        if visitors == 0 {
            ctx.log_warning(&format!(
                "visitors 0 for mcaptcha mcaptcha {}.",
                mcaptcha_name,
            ));
            return;
        }

        match Bucket::increment_by(ctx, (mcaptcha_name.to_owned(), duration), visitors) {
            Err(e) => ctx.log_warning(&format!("{}", e)),
            Ok(()) => ctx.log_debug(&format!(
                "Created new bucket making captcha {} eventually consistent",
                &mcaptcha_name
            )),
        }
    }

    #[allow(clippy::new_ret_no_self)]
    pub fn new(ctx: &Context, duration: u64, mcaptcha_name: &str) -> CacheResult<()> {
        let safety_name = get_safety_name(mcaptcha_name);
        let safety = ctx.open_key_writable(&safety_name);

        if safety.key_type() == KeyType::Empty {
            let safety_val = MCaptchaSafety {};
            safety.set_value(&MCAPTCHA_SAFETY_TYPE, safety_val)?;
            ctx.log_debug(&format!("mcaptcha safety created: {}", safety_name));
            Self::set_timer(ctx, &safety, (safety_name, duration))?;
        } else {
            ctx.log_debug(&format!("mcaptcha safety exists: {}", safety_name));
        }
        Ok(())
    }

    fn set_timer(
        ctx: &Context,
        safety: &RedisKeyWritable,
        (safety_name, duration): (String, u64),
    ) -> CacheResult<()> {
        let _ = ctx.create_timer(
            Duration::from_secs(duration),
            Self::boost,
            (safety_name, duration),
        );
        safety.set_expire(Duration::from_secs(duration * 2))?;
        Ok(())
    }

    /// executes when timer goes off. Refreshes expiry timer and resets timer
    fn boost(ctx: &Context, (safety_name, duration): (String, u64)) {
        let safety = ctx.open_key_writable(&safety_name);

        match safety.get_value::<Self>(&MCAPTCHA_SAFETY_TYPE) {
            Ok(Some(_safety_val)) => match Self::set_timer(ctx, &safety, (safety_name, duration)) {
                Ok(_) => (),
                Err(e) => ctx.log_warning(&format!("{}", e)),
            },
            _ => {
                let mcaptcha_name = get_mcaptcha_from_safety(&safety_name);
                if mcaptcha_name.is_none() {
                    return;
                }
                let mcaptcha_name = mcaptcha_name.unwrap();
                let mcaptcha = ctx.open_key(&mcaptcha_name);
                if mcaptcha.key_type() == KeyType::Empty {
                    return;
                }

                if let Ok(Some(_)) = MCaptcha::get_mcaptcha(&mcaptcha) {
                    let res = Self::new(ctx, duration, mcaptcha_name);
                    if res.is_err() {
                        ctx.log_warning(&format!(
                            "Error when creating safety timer for mcaptcha key: {}. Error: {}",
                            mcaptcha_name,
                            res.err().unwrap()
                        ));
                    }
                }
            }
        }
    }
}

pub static MCAPTCHA_SAFETY_TYPE: RedisType = RedisType::new(
    "mcaptsafe",
    MCAPTCHA_SAFETY_VERSION,
    raw::RedisModuleTypeMethods {
        version: raw::REDISMODULE_TYPE_METHOD_VERSION as u64,
        rdb_load: Some(type_methods::rdb_load),
        rdb_save: Some(type_methods::rdb_save),
        aof_rewrite: None,
        free: Some(type_methods::free),

        // Currently unused by Redis
        mem_usage: None,
        digest: None,

        // Aux data
        aux_load: None,
        aux_save: None,
        aux_save_triggers: 0,

        free_effort: None,
        unlink: None,
        copy: None,
        defrag: None,
    },
);

pub mod type_methods {
    use std::os::raw::c_void;

    use libc::c_int;

    const SAFETY_RDB_VAL: &str = "SAFETY";

    use super::*;
    #[allow(non_snake_case, unused)]
    pub extern "C" fn rdb_load(rdb: *mut raw::RedisModuleIO, encver: c_int) -> *mut c_void {
        let bucket = match encver {
            0 => {
                let data = raw::load_string(rdb);
                if data == SAFETY_RDB_VAL {
                    MCaptchaSafety {}
                } else {
                    panic!("Can't safety from old redis RDB, data received : {}", data);
                }
            }
            _ => panic!(
                "Can't safety from old redis RDB, encoding version: {}",
                encver
            ),
        };

        //        if bucket.
        Box::into_raw(Box::new(bucket)) as *mut c_void
    }

    pub unsafe extern "C" fn free(value: *mut c_void) {
        let val = value as *mut MCaptchaSafety;
        Box::from_raw(val);
    }

    #[allow(non_snake_case, unused)]
    pub unsafe extern "C" fn rdb_save(rdb: *mut raw::RedisModuleIO, value: *mut c_void) {
        raw::save_string(rdb, &SAFETY_RDB_VAL)
    }
}
