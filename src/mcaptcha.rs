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
use libmcaptcha::dev::{AddVisitorResult, CreateMCaptcha, DefenseBuilder, MCaptchaBuilder};
use redis_module::key::RedisKey;
use redis_module::key::RedisKeyWritable;
use redis_module::native_types::RedisType;
use redis_module::raw::KeyType;
use redis_module::RedisError;
use redis_module::RedisString;
use redis_module::RedisValue;
use redis_module::{Context, RedisResult};
use redis_module::{NextArg, REDIS_OK};
//use redis_module::RedisError;
use redis_module::raw;

use serde::{Deserialize, Serialize};

use crate::bucket::Format;
use crate::errors::*;
use crate::safety::MCaptchaSafety;
use crate::utils::*;

const REDIS_MCPATCHA_MCAPTCHA_TYPE_VERSION: i32 = 0;

#[derive(Serialize, Deserialize)]
pub struct MCaptcha {
    m: libmcaptcha::dev::MCaptcha,
}

impl MCaptcha {
    #[inline]
    pub fn get_add_visitor_result(&self) -> AddVisitorResult {
        AddVisitorResult::new(&self.m)
    }

    #[inline]
    fn new(mut m: CreateMCaptcha) -> CacheResult<Self> {
        let mut defense_builder = DefenseBuilder::default();
        for l in m.levels.drain(0..) {
            defense_builder.add_level(l)?;
        }
        let defense = defense_builder.build()?;

        let m = MCaptchaBuilder::default()
            .defense(defense)
            .duration(m.duration)
            .build()?;

        Ok(MCaptcha { m })
    }

    /// increments the visitor count by one
    #[inline]
    pub fn add_visitor(&mut self) {
        self.m.add_visitor()
    }

    /// get current difficulty factor
    #[inline]
    #[allow(dead_code)]
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
    pub fn get_mut_mcaptcha(key: &RedisKeyWritable) -> CacheResult<Option<&mut Self>> {
        Ok(key.get_value::<Self>(&MCAPTCHA_MCAPTCHA_TYPE)?)
    }

    /// get mcaptcha from redis key
    #[inline]
    pub fn get_mcaptcha(key: &RedisKey) -> CacheResult<Option<&Self>> {
        Ok(key.get_value::<Self>(&MCAPTCHA_MCAPTCHA_TYPE)?)
    }

    /// Get counter value
    pub fn get_count(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
        let mut args = args.into_iter().skip(1);
        let key_name = args.next_string()?;
        let key_name = get_captcha_key(&key_name);

        let stored_captcha = ctx.open_key(&RedisString::create_from_slice(
            ctx.ctx,
            key_name.as_bytes(),
        ));
        if stored_captcha.key_type() == KeyType::Empty {
            return CacheError::new(format!("key {} not found", key_name)).into();
        }

        match Self::get_mcaptcha(&stored_captcha)? {
            Some(val) => Ok(RedisValue::Integer(val.get_visitors().into())),
            None => Err(CacheError::CaptchaNotFound.into()),
        }
    }

    /// Add captcha to redis
    pub fn add_captcha(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
        let mut args = args.into_iter().skip(1);
        let key_name = get_captcha_key(&args.next_string()?);
        let json = args.next_string()?;
        let mcaptcha: CreateMCaptcha = Format::Json.from_str(&json)?;
        let mcaptcha = Self::new(mcaptcha)?;

        Self::add_captcha_runner(ctx, &key_name, mcaptcha)
    }

    #[inline]
    fn add_captcha_runner(ctx: &Context, key_name: &str, mcaptcha: MCaptcha) -> RedisResult {
        let duration = mcaptcha.get_duration();
        let key = ctx.open_key_writable(&RedisString::create_from_slice(
            ctx.ctx,
            key_name.as_bytes(),
        ));
        if key.key_type() == KeyType::Empty {
            key.set_value(&MCAPTCHA_MCAPTCHA_TYPE, mcaptcha)?;
            ctx.log_debug(&format!("mcaptcha {} created", key_name));
            MCaptchaSafety::new(ctx, duration, key_name)?;
            REDIS_OK
        } else {
            let msg = format!("mcaptcha {} exists", key_name);
            ctx.log_debug(&msg);
            Err(CacheError::new(msg).into())
        }
    }

    /// check if captcha exists
    pub fn captcha_exists(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
        let mut args = args.into_iter().skip(1);
        let key_name = get_captcha_key(&args.next_string()?);

        let key = ctx.open_key(&RedisString::create_from_slice(
            ctx.ctx,
            key_name.as_bytes(),
        ));
        if Self::captcha_exists_runner(&key) {
            Ok(RedisValue::Integer(0))
        } else {
            Ok(RedisValue::Integer(1))
        }
    }

    #[inline]
    fn captcha_exists_runner(key: &RedisKey) -> bool {
        !(key.key_type() == KeyType::Empty)
    }

    /// implements mCaptcha rename: clones configuration from old name to new name and
    /// deletes oldname
    pub fn rename(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
        let mut args = args.into_iter().skip(1);
        let key_name = get_captcha_key(&args.next_string()?);
        let new_name = get_captcha_key(&args.next_string()?);

        let key = ctx.open_key(&RedisString::create_from_slice(
            ctx.ctx,
            key_name.as_bytes(),
        ));
        if Self::captcha_exists_runner(&key) {
            if let Some(mcaptcha) = Self::get_mcaptcha(&key)? {
                let mcaptcha = MCaptcha {
                    m: MCaptchaBuilder::default()
                        .defense(mcaptcha.m.get_defense())
                        .duration(mcaptcha.get_duration())
                        .build()?,
                };

                Self::add_captcha_runner(ctx, &new_name, mcaptcha)?;
                Self::delete_captcha_runner(ctx, &key_name)?;
            }
        };

        REDIS_OK
    }

    /// delete captcha
    pub fn delete_captcha(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
        let mut args = args.into_iter().skip(1);
        let key_name = get_captcha_key(&args.next_string()?);
        Self::delete_captcha_runner(ctx, &key_name)
    }

    #[inline]
    fn delete_captcha_runner(ctx: &Context, key_name: &str) -> RedisResult {
        let key = ctx.open_key_writable(&RedisString::create_from_slice(
            ctx.ctx,
            key_name.as_bytes(),
        ));
        if key.key_type() == KeyType::Empty {
            Err(RedisError::nonexistent_key())
        } else {
            key.delete()?;
            REDIS_OK
        }
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
        mem_usage2: None,
        digest: None,

        // Aux data
        aux_load: None,
        aux_save: None,
        aux_save2: None,
        aux_save_triggers: 0,

        free_effort: None,
        free_effort2: None,
        unlink: None,
        unlink2: None,
        copy: None,
        copy2: None,
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
                let data = raw::load_string(rdb).unwrap().to_string();
                let mcaptcha: Result<MCaptcha, CacheError> = Format::Json.from_str(&data);
                if mcaptcha.is_err() {
                    panic!(
                        "Can't load mCaptcha from old redis RDB, error while serde {}, data received: {}",
                        mcaptcha.err().unwrap(),
                        data
                    );
                }
                mcaptcha.unwrap()
            }
            _ => panic!("Can't load mCaptcha from old redis RDB, encver {}", encver),
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
            Ok(string) => raw::save_string(rdb, string),
            Err(e) => panic!("error while rdb_save: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use libmcaptcha::defense::Level;
    use libmcaptcha::defense::LevelBuilder;

    fn get_levels() -> Vec<Level> {
        vec![
            LevelBuilder::default()
                .visitor_threshold(50)
                .difficulty_factor(50)
                .unwrap()
                .build()
                .unwrap(),
            LevelBuilder::default()
                .visitor_threshold(500)
                .difficulty_factor(5000)
                .unwrap()
                .build()
                .unwrap(),
            LevelBuilder::default()
                .visitor_threshold(5000)
                .difficulty_factor(50000)
                .unwrap()
                .build()
                .unwrap(),
            LevelBuilder::default()
                .visitor_threshold(50000)
                .difficulty_factor(500000)
                .unwrap()
                .build()
                .unwrap(),
            LevelBuilder::default()
                .visitor_threshold(500000)
                .difficulty_factor(5000000)
                .unwrap()
                .build()
                .unwrap(),
        ]
    }

    #[test]
    fn create_mcaptcha_works() {
        let levels = get_levels();
        let payload = CreateMCaptcha {
            levels,
            duration: 30,
        };

        let mcaptcha = MCaptcha::new(payload);
        assert!(mcaptcha.is_ok());
        let mut mcaptcha = mcaptcha.unwrap();

        for _ in 0..50 {
            mcaptcha.add_visitor();
        }
        assert_eq!(mcaptcha.get_visitors(), 50);
        assert_eq!(mcaptcha.get_difficulty(), 50);

        for _ in 0..451 {
            mcaptcha.add_visitor();
        }
        assert_eq!(mcaptcha.get_visitors(), 501);
        assert_eq!(mcaptcha.get_difficulty(), 5000);

        mcaptcha.decrement_visitor_by(501);
        for _ in 0..5002 {
            mcaptcha.add_visitor();
        }
        assert_eq!(mcaptcha.get_visitors(), 5002);
        assert_eq!(mcaptcha.get_difficulty(), 50000);
    }
}
