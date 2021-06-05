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

use std::collections::HashMap;
use std::os::raw::c_void;
use std::time::Duration;
//use std::time::{SystemTime, UNIX_EPOCH};

use redis_module::key::RedisKeyWritable;
use redis_module::native_types::RedisType;
use redis_module::raw::KeyType;
use redis_module::NotifyEvent;
use redis_module::RedisError;
use redis_module::{raw, Context};
use serde::{Deserialize, Serialize};

use crate::errors::CacheError;
use crate::utils::*;
use crate::*;

#[derive(Debug, PartialEq)]
/// encoding formats for persistence
pub enum Format {
    JSON,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pocket {
    /// timer ID
    timer: u64,
    /// instant(seconds from UNIX_EPOCH) at which time pocket begins decrement process
    pocket_instant: u64,
    /// a list of captcha keys that should be decremented during clean up
    decrement: HashMap<String, usize>,
}

impl Pocket {
    pub fn on_delete(ctx: &Context, event_type: NotifyEvent, event: &str, key_name: &str) {
        let msg = format!(
            "Received event: {:?} on key: {} via event: {}",
            event_type, key_name, event
        );
        ctx.log_debug(msg.as_str());

        if !is_pocket_timer(key_name) {
            return;
        }

        let pocket_name = get_pocket_name_from_timer_name(key_name);
        if pocket_name.is_none() {
            return;
        }

        let pocket_name = pocket_name.unwrap();

        let pocket = ctx.open_key_writable(&pocket_name);
        if pocket.key_type() == KeyType::Empty {
            ctx.log_debug(&format!("Pocket doesn't exist: {}", &key_name));
            return;
        } else {
            Pocket::decrement_runner(ctx, &pocket);
        }
    }

    /// creates new pocket and sets off timer to go off at `duration`
    #[inline]
    pub fn new(ctx: &Context, duration: u64) -> Result<Self, RedisError> {
        let decrement = HashMap::with_capacity(1);

        let pocket_instant = get_pocket_instant(duration)?;
        let timer = ctx.create_timer(
            Duration::from_secs(duration),
            Self::decrement,
            pocket_instant,
        );

        let pocket = Pocket {
            timer,
            pocket_instant,
            decrement,
        };
        Ok(pocket)
    }

    /// increments count of key = captcha and registers for auto decrement
    #[inline]
    pub fn increment(ctx: &Context, duration: u64, captcha: &str) -> Result<(), RedisError> {
        let captcha_name = get_captcha_key(captcha);
        ctx.log_debug(&captcha_name);
        // increment
        let captcha = ctx.open_key_writable(&captcha_name);

        match captcha.read()? {
            Some(val) => {
                if val.trim().is_empty() {
                    captcha.write("1")?;
                } else {
                    let mut val: usize = val.parse()?;
                    val += 1;
                    captcha.write(&val.to_string())?;
                }
            }
            None => {
                captcha.write("1")?;
            }
        }

        let pocket_instant = get_pocket_instant(duration)?;
        let pocket_name = get_pocket_name(pocket_instant);

        ctx.log_debug(&format!("Pocket name: {}", &pocket_name));

        // get  pocket
        let pocket = ctx.open_key_writable(&pocket_name);

        match pocket.get_value::<Pocket>(&MCAPTCHA_POCKET_TYPE)? {
            Some(pocket) => match pocket.decrement.get_mut(&captcha_name) {
                Some(count) => *count += 1,
                None => {
                    pocket.decrement.insert(captcha_name, 1);
                }
            },

            None => {
                let mut counter = Pocket::new(ctx, duration)?;
                counter.decrement.insert(captcha_name, 1);
                pocket.set_value(&MCAPTCHA_POCKET_TYPE, counter)?;
                let timer = ctx.open_key_writable(&get_timer_name_from_pocket_name(&pocket_name));
                timer.write("1")?;
                timer.set_expire(Duration::from_secs(duration + POCKET_EXPIRY_OFFSET))?;
            }
        };

        Ok(())
    }

    /// decrement runner that decrements all registered counts _without_ cleaning after itself
    /// use [decrement] when you require auto cleanup. Internally, it calls this method.
    #[inline]
    fn decrement_runner(ctx: &Context, key: &RedisKeyWritable) {
        let val = key.get_value::<Pocket>(&MCAPTCHA_POCKET_TYPE).unwrap();
        match val {
            Some(pocket) => {
                ctx.log_debug(&format!("entering loop hashmap "));
                for (captcha, count) in pocket.decrement.iter() {
                    ctx.log_debug(&format!(
                        "reading captcha: {} with decr count {}",
                        &captcha, count
                    ));
                    let stored_captcha = ctx.open_key_writable(&captcha);
                    if stored_captcha.key_type() == KeyType::Empty {
                        continue;
                    }

                    let mut stored_count: usize =
                        stored_captcha.read().unwrap().unwrap().parse().unwrap();
                    stored_count -= count;
                    if stored_count == 0 {
                        match stored_captcha.delete() {
                            Err(e) => ctx.log_warning(&format!(
                                "Error occured while cleaning up captcha when it became 0: {}",
                                e
                            )),
                            Ok(_) => (),
                        }
                    } else {
                        stored_captcha.write(&stored_count.to_string()).unwrap();
                    }
                }
            }
            None => {
                ctx.log_debug(&format!("pocket not found, can't decrement"));
            }
        }
    }

    /// executes when timer goes off. Decrements all registered counts and cleans itself up
    fn decrement(ctx: &Context, pocket_instant: u64) {
        // get  pocket
        let pocket_name = get_pocket_name(pocket_instant);

        let timer = ctx.open_key_writable(&get_timer_name_from_pocket_name(&pocket_name));
        let _ = timer.delete();

        ctx.log_debug(&format!("Pocket instant: {}", &pocket_instant));

        let pocket = ctx.open_key_writable(&pocket_name);
        Pocket::decrement_runner(ctx, &pocket);

        match pocket.delete() {
            Err(e) => ctx.log_warning(&format!("enountered error while deleting hashmap: {:?}", e)),
            Ok(_) => (),
        }
    }

    #[inline]
    pub fn parse_str(data: &str, format: Format) -> Result<Pocket, CacheError> {
        match format {
            Format::JSON => Ok(serde_json::from_str(data)?),
        }
    }

    #[inline]
    pub fn from_str(data: &str, format: Format) -> Result<Self, CacheError> {
        Ok(Pocket::parse_str(data, format)?)
    }
}

pub static MCAPTCHA_POCKET_TYPE: RedisType = RedisType::new(
    "mcaptchac",
    REDIS_MCAPTCHA_POCKET_TYPE_VERSION,
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
    use libc::c_int;

    use super::*;

    #[allow(non_snake_case, unused)]
    pub extern "C" fn rdb_load(rdb: *mut raw::RedisModuleIO, encver: c_int) -> *mut c_void {
        let pocket = match encver {
            0 => {
                let data = raw::load_string(rdb);
                Pocket::from_str(&data, Format::JSON).unwrap()
            }
            _ => panic!("Can't load old RedisJSON RDB"),
        };

        //        if pocket.
        Box::into_raw(Box::new(pocket)) as *mut c_void
    }

    pub unsafe extern "C" fn free(value: *mut c_void) {
        let val = value as *mut Pocket;
        Box::from_raw(val);
    }

    #[allow(non_snake_case, unused)]
    pub unsafe extern "C" fn rdb_save(rdb: *mut raw::RedisModuleIO, value: *mut c_void) {
        let pocket = &*(value as *mut Pocket);
        match &serde_json::to_string(pocket) {
            Ok(string) => raw::save_string(rdb, &string),
            Err(e) => eprintln!("error while rdb_save: {}", e),
        }
    }
}
