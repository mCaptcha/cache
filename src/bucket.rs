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
use std::time::Duration;
//use std::time::{SystemTime, UNIX_EPOCH};

use redis_module::key::RedisKeyWritable;
use redis_module::native_types::RedisType;
use redis_module::raw::KeyType;
use redis_module::NotifyEvent;
//use redis_module::RedisError;
use redis_module::{raw, Context};
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::mcaptcha::MCaptcha;
use crate::utils::*;
use crate::*;

/// Bucket type version, aka encoding version
const REDIS_MCAPTCHA_BUCKET_TYPE_VERSION: i32 = 0;

#[derive(Debug, PartialEq)]
/// encoding formats for persistence
pub enum Format {
    JSON,
}

impl Format {
    #[inline]
    pub fn parse_str<'a, T: Deserialize<'a>>(&self, data: &'a str) -> CacheResult<T> {
        match self {
            Format::JSON => Ok(serde_json::from_str(data)?),
        }
    }

    #[inline]
    pub fn from_str<'a, T: Deserialize<'a>>(&self, data: &'a str) -> CacheResult<T> {
        Ok(self.parse_str(data)?)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Bucket {
    /// timer ID
    timer: u64,
    /// instant(seconds from UNIX_EPOCH) at which time bucket begins decrement process
    bucket_instant: u64,
    /// a list of captcha keys that should be decremented during clean up
    decrement: HashMap<String, u32>,
}

impl Bucket {
    pub fn on_delete(ctx: &Context, event_type: NotifyEvent, event: &str, key_name: &str) {
        let msg = format!(
            "Received event: {:?} on key: {} via event: {}",
            event_type, key_name, event
        );
        ctx.log_debug(msg.as_str());

        if !is_bucket_timer(key_name) {
            return;
        }

        let bucket_name = get_bucket_name_from_timer_name(key_name);
        if bucket_name.is_none() {
            return;
        }

        let bucket_name = bucket_name.unwrap();

        let bucket = ctx.open_key_writable(&bucket_name);
        if bucket.key_type() == KeyType::Empty {
            ctx.log_debug(&format!("Bucket doesn't exist: {}", &key_name));
            return;
        } else {
            Bucket::decrement_runner(ctx, &bucket);
        }
    }

    /// creates new bucket and sets off timer to go off at `duration`
    #[inline]
    fn new(ctx: &Context, duration: u64) -> CacheResult<Self> {
        let decrement = HashMap::with_capacity(HIT_PER_SECOND);

        let bucket_instant = get_bucket_instant(duration)?;
        let timer = ctx.create_timer(
            Duration::from_secs(duration),
            Self::decrement,
            bucket_instant,
        );

        let bucket = Bucket {
            timer,
            bucket_instant,
            decrement,
        };
        Ok(bucket)
    }

    /// decrement runner that decrements all registered counts _without_ cleaning after itself
    /// use [decrement] when you require auto cleanup. Internally, it calls this method.
    #[inline]
    fn decrement_runner(ctx: &Context, key: &RedisKeyWritable) {
        match key.get_value::<Bucket>(&MCAPTCHA_BUCKET_TYPE) {
            Ok(Some(bucket)) => {
                ctx.log_debug(&format!("entering loop hashmap "));
                for (captcha, count) in bucket.decrement.drain() {
                    ctx.log_debug(&format!(
                        "reading captcha: {} with decr count {}",
                        &captcha, count
                    ));
                    let stored_captcha = ctx.open_key_writable(&captcha);
                    if stored_captcha.key_type() == KeyType::Empty {
                        continue;
                    }
                    if let Ok(Some(captcha)) = MCaptcha::get_mut_mcaptcha(&stored_captcha) {
                        captcha.decrement_visitor_by(count);
                    }
                }
            }
            _ => {
                ctx.log_debug(&format!("bucket not found, can't decrement"));
            }
        }
    }

    /// executes when timer goes off. Decrements all registered counts and cleans itself up
    fn decrement(ctx: &Context, bucket_instant: u64) {
        // get  bucket
        let bucket_name = get_bucket_name(bucket_instant);

        let timer = ctx.open_key_writable(&get_timer_name_from_bucket_name(&bucket_name));
        let _ = timer.delete();

        ctx.log_debug(&format!("Bucket instant: {}", &bucket_instant));

        let bucket = ctx.open_key_writable(&bucket_name);
        Bucket::decrement_runner(ctx, &bucket);

        match bucket.delete() {
            Err(e) => ctx.log_warning(&format!("enountered error while deleting hashmap: {:?}", e)),
            Ok(_) => (),
        }
    }

    /// increments count of key = captcha and registers for auto decrement
    #[inline]
    fn increment(ctx: &Context, captcha: &str) -> CacheResult<()> {
        let captcha_name = get_captcha_key(captcha);
        ctx.log_debug(&captcha_name);
        // increment
        let captcha = ctx.open_key_writable(&captcha_name);
        ctx.log_debug("loading mcaptcha");
        let captcha = MCaptcha::get_mut_mcaptcha(&captcha)?;

        ctx.log_debug("loaded mcaptcha");
        if captcha.is_none() {
            return Err(CacheError::new("Captcha not found".into()));
        }
        let captcha = captcha.unwrap();
        ctx.log_debug(&format!(
            "current visitor count: {}",
            captcha.get_visitors()
        ));
        captcha.add_visitor();

        ctx.log_debug("visitor added");
        let duration = captcha.get_duration();
        let bucket_instant = get_bucket_instant(duration)?;
        let bucket_name = get_bucket_name(bucket_instant);

        ctx.log_debug(&format!("Bucket name: {}", &bucket_name));

        // get  bucket
        let bucket = ctx.open_key_writable(&bucket_name);

        match bucket.get_value::<Bucket>(&MCAPTCHA_BUCKET_TYPE)? {
            Some(bucket) => match bucket.decrement.get_mut(&captcha_name) {
                Some(count) => *count += 1,
                None => {
                    bucket.decrement.insert(captcha_name, 1);
                }
            },

            None => {
                let mut counter = Bucket::new(ctx, duration)?;
                counter.decrement.insert(captcha_name, 1);
                bucket.set_value(&MCAPTCHA_BUCKET_TYPE, counter)?;
                let timer = ctx.open_key_writable(&get_timer_name_from_bucket_name(&bucket_name));
                timer.write("1")?;
                timer.set_expire(Duration::from_secs(duration + BUCKET_EXPIRY_OFFSET))?;
            }
        };

        Ok(())
    }

    /// Create new counter
    pub fn counter_create(ctx: &Context, args: Vec<String>) -> RedisResult {
        let mut args = args.into_iter().skip(1);
        // mcaptcha captcha key name
        let key_name = args.next_string()?;
        // expiry
        bucket::Bucket::increment(ctx, &key_name)?;
        REDIS_OK
    }
}

pub static MCAPTCHA_BUCKET_TYPE: RedisType = RedisType::new(
    "mcaptbuck",
    REDIS_MCAPTCHA_BUCKET_TYPE_VERSION,
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

    use super::*;

    #[allow(non_snake_case, unused)]
    pub extern "C" fn rdb_load(rdb: *mut raw::RedisModuleIO, encver: c_int) -> *mut c_void {
        let bucket = match encver {
            0 => {
                let data = raw::load_string(rdb);
                let bucket: Bucket = Format::JSON.from_str(&data).unwrap();
                bucket
            }
            _ => panic!("Can't load bucket from old redis RDB, encver: {}", encver,),
        };

        //        if bucket.
        Box::into_raw(Box::new(bucket)) as *mut c_void
    }

    pub unsafe extern "C" fn free(value: *mut c_void) {
        let val = value as *mut Bucket;
        Box::from_raw(val);
    }

    #[allow(non_snake_case, unused)]
    pub unsafe extern "C" fn rdb_save(rdb: *mut raw::RedisModuleIO, value: *mut c_void) {
        let bucket = &*(value as *mut Bucket);
        match &serde_json::to_string(bucket) {
            Ok(string) => raw::save_string(rdb, &string),
            Err(e) => eprintln!("error while rdb_save: {}", e),
        }
    }
}
