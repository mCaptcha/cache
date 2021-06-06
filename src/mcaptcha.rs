use redis_module::key::RedisKey;
use redis_module::RedisValue;
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
use redis_module::key::RedisKeyWritable;
use redis_module::native_types::RedisType;
use redis_module::raw::KeyType;
use redis_module::{Context, RedisResult};
use redis_module::{NextArg, REDIS_OK};
//use redis_module::RedisError;
use redis_module::raw;

use serde::{Deserialize, Serialize};

use crate::bucket::Format;
use crate::errors::*;
use crate::utils::*;

const REDIS_MCPATCHA_MCAPTCHA_TYPE_VERSION: i32 = 0;

#[derive(Serialize, Deserialize)]
pub struct MCaptcha {
    m: libmcaptcha::MCaptcha,
}

impl MCaptcha {
    #[inline]
    fn new(m: libmcaptcha::MCaptcha) -> Self {
        MCaptcha { m }
    }
    /// increments the visitor count by one
    #[inline]
    pub fn add_visitor(&mut self) {
        self.m.add_visitor()
    }

    /// decrements the visitor count by one
    #[inline]
    pub fn decrement_visitor(&mut self) {
        self.m.decrement_visitor()
    }

    /// get current difficulty factor
    #[inline]
    pub fn get_difficulty(&self) -> u32 {
        self.m.get_difficulty()
    }

    /// get [MCaptcha]'s lifetime
    #[inline]
    pub fn get_duration(&self) -> u64 {
        self.m.get_duration()
    }

    /// get [MCaptcha]'s current visitor_threshold
    #[inline]
    pub fn get_visitors(&self) -> u32 {
        self.m.get_visitors()
    }

    /// decrement [MCaptcha]'s current visitor_threshold by specified count
    #[inline]
    pub fn decrement_visitor_by(&mut self, count: u32) {
        self.m.decrement_visitor_by(count)
    }

    /// get mcaptcha from redis key writable
    #[inline]
    pub fn get_mut_mcaptcha<'a>(key: &'a RedisKeyWritable) -> CacheResult<Option<&'a mut Self>> {
        Ok(key.get_value::<Self>(&MCAPTCHA_MCAPTCHA_TYPE)?)
    }

    /// get mcaptcha from redis key
    #[inline]
    pub fn get_mcaptcha<'a>(key: &'a RedisKey) -> CacheResult<Option<&'a Self>> {
        Ok(key.get_value::<Self>(&MCAPTCHA_MCAPTCHA_TYPE)?)
    }

    /// Get counter value
    pub fn get_count(ctx: &Context, args: Vec<String>) -> RedisResult {
        let mut args = args.into_iter().skip(1);
        let key_name = args.next_string()?;
        let key_name = get_captcha_key(&key_name);

        let stored_captcha = ctx.open_key(&key_name);
        if stored_captcha.key_type() == KeyType::Empty {
            return CacheError::new(format!("key {} not found", key_name)).into();
        }

        match stored_captcha.get_value::<Self>(&MCAPTCHA_MCAPTCHA_TYPE)? {
            Some(val) => Ok(RedisValue::Integer(val.get_visitors().into())),
            None => return Err(CacheError::CaptchaNotFound.into()),
        }
    }

    /// Add captcha to redis
    pub fn add_captcha(ctx: &Context, args: Vec<String>) -> RedisResult {
        let mut args = args.into_iter().skip(1);
        let key_name = get_captcha_key(&args.next_string()?);
        let json = args.next_string()?;
        let mcaptcha: libmcaptcha::MCaptcha = Format::JSON.from_str(&json)?;
        let mcaptcha = Self::new(mcaptcha);

        let key = ctx.open_key_writable(&&key_name);
        key.set_value(&MCAPTCHA_MCAPTCHA_TYPE, mcaptcha)?;

        REDIS_OK
    }
}

pub static MCAPTCHA_MCAPTCHA_TYPE: RedisType = RedisType::new(
    "mcaptmcap",
    REDIS_MCPATCHA_MCAPTCHA_TYPE_VERSION,
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
        let mcaptcha = match encver {
            0 => {
                let data = raw::load_string(rdb);
                let mcaptcha: MCaptcha = Format::JSON.from_str(&data).unwrap();
                mcaptcha
            }
            _ => panic!("Can't load mCaptcha from old redis RDB"),
        };

        Box::into_raw(Box::new(mcaptcha)) as *mut c_void
    }

    pub unsafe extern "C" fn free(value: *mut c_void) {
        let val = value as *mut MCaptcha;
        Box::from_raw(val);
    }

    #[allow(non_snake_case, unused)]
    pub unsafe extern "C" fn rdb_save(rdb: *mut raw::RedisModuleIO, value: *mut c_void) {
        let mcaptcha = &*(value as *mut MCaptcha);
        match &serde_json::to_string(mcaptcha) {
            Ok(string) => raw::save_string(rdb, &string),
            Err(e) => panic!("error while rdb_save: {}", e),
        }
    }
}
