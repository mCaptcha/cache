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
//use redis_module::RedisError;
use redis_module::{raw, Context};
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::mcaptcha::MCaptcha;
use crate::utils::*;

const MCAPTCHA_SAFETY_VERSION: i32 = 0;

#[derive(Serialize, Deserialize)]
pub struct MCaptchaSafety;

impl MCaptchaSafety {
    pub fn new(ctx: &Context, duration: u64, mcaptcha_name: &str) -> CacheResult<()> {
        let safety_name = get_safety_name(mcaptcha_name);
        let safety = ctx.open_key_writable(&safety_name);
        Self::set_timer(ctx, &safety, (&safety_name, duration))?;
        Ok(())
    }

    fn set_timer(
        ctx: &Context,
        safety: &RedisKeyWritable,
        (safety_name, duration): (&str, u64),
    ) -> CacheResult<()> {
        let _ = ctx.create_timer(
            Duration::from_secs(duration),
            Self::boost,
            (&safety_name, duration),
        );

        safety.set_expire(Duration::from_secs(duration * 2))?;
        Ok(())
    }

    /// executes when timer goes off. Refreshes expiry timer and resets timer
    fn boost(ctx: &Context, (safety_name, duration): (&str, u64)) {
        let safety = ctx.open_key_writable(safety_name);

        let x = safety.get_value::<Self>(&MCAPTCHA_SAFETY_TYPE);
        // Result<Option<&mut Safety>, RedisError>
        // Ok(Some(val)) => refresh
        // _ => check if corresponding captcha is available => Yes -> create timer
        //                                                     NO -> Ignore
        //

        match safety.get_value::<Self>(&MCAPTCHA_SAFETY_TYPE) {
            Ok(Some(_safety_val)) => {
                Self::set_timer(ctx, &safety, (&safety_name, duration)).unwrap()
            }
            _ => {
                let mcaptcha_name = get_mcaptcha_from_safety(safety_name);
                if mcaptcha_name.is_none() {
                    return;
                }
                let mcaptcha_name = mcaptcha_name.unwrap();
                let mcaptcha = ctx.open_key(&mcaptcha_name);
                if mcaptcha.key_type() == KeyType::Empty {
                    return;
                }

                if let Ok(Some(val)) = MCaptcha::get_mcaptcha(&mcaptcha) {
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
    "mcaptdafe",
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

    use crate::bucket::Format;

    use super::*;

    #[allow(non_snake_case, unused)]
    pub extern "C" fn rdb_load(rdb: *mut raw::RedisModuleIO, encver: c_int) -> *mut c_void {
        let bucket = match encver {
            0 => {
                let data = raw::load_string(rdb);
                let bucket: MCaptchaSafety = Format::JSON.from_str(&data).unwrap();
                bucket
            }
            _ => panic!("Can't load bucket from old redis RDB"),
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
        let bucket = &*(value as *mut MCaptchaSafety);
        match &serde_json::to_string(bucket) {
            Ok(string) => raw::save_string(rdb, &string),
            Err(e) => eprintln!("error while rdb_save: {}", e),
        }
    }
}
