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

use redis_module::native_types::RedisType;
use redis_module::raw::KeyType;
use redis_module::NextArg;
use redis_module::RedisResult;
use redis_module::REDIS_OK;
use redis_module::{raw, Context};
use serde::{Deserialize, Serialize};

use crate::bucket::Format;
use crate::errors::*;
use crate::utils::*;

const MCAPTCHA_CHALLENGE_VERSION: i32 = 0;

#[derive(Serialize, Deserialize)]
pub struct Challenge {
    difficulty: usize,
    duration: u64,
}

#[derive(Serialize, Deserialize)]
pub struct AddChallenge {
    difficulty: usize,
    duration: u64,
    challenge: String,
}

impl Challenge {
    pub fn new(duration: u64, difficulty: usize) -> Self {
        Self {
            difficulty,
            duration,
        }
    }

    pub fn create_challenge(ctx: &Context, args: Vec<String>) -> RedisResult {
        let mut args = args.into_iter().skip(1);
        let captcha = args.next_string()?;
        let json = args.next_string()?;
        let add_challenge: AddChallenge = Format::JSON.from_str(&json)?;

        let challenge_name = get_challenge_name(&captcha, &add_challenge.challenge);

        let key = ctx.open_key_writable(&challenge_name);
        if key.key_type() != KeyType::Empty {
            return Err(CacheError::DuplicateChallenge.into());
        }
        let challenge = Self::new(add_challenge.duration, add_challenge.difficulty);

        key.set_value(&MCAPTCHA_CHALLENGE_TYPE, challenge)?;
        key.set_expire(Duration::from_secs(add_challenge.duration))?;

        REDIS_OK
    }

    pub fn get_challenge(ctx: &Context, args: Vec<String>) -> RedisResult {
        let mut args = args.into_iter().skip(1);
        let captcha = args.next_string()?;
        let challenge = args.next_string()?;

        let challenge_name = get_challenge_name(&captcha, &challenge);

        let key = ctx.open_key_writable(&challenge_name);
        if key.key_type() == KeyType::Empty {
            return Err(CacheError::ChallengeNotFound.into());
        }
        match key.get_value::<Self>(&MCAPTCHA_CHALLENGE_TYPE)? {
            Some(challenge) => {
                let resp = serde_json::to_string(&challenge)?;
                key.delete()?;
                Ok(resp.into())
            }
            None => Err(CacheError::ChallengeNotFound.into()),
        }
    }
}

pub static MCAPTCHA_CHALLENGE_TYPE: RedisType = RedisType::new(
    "mcaptchal",
    MCAPTCHA_CHALLENGE_VERSION,
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
        let challenge = match encver {
            0 => {
                let data = raw::load_string(rdb);
                let challenge: Result<Challenge, CacheError> = Format::JSON.from_str(&data);
                if challenge.is_err() {
                    panic!(
                        "Can't load Challenge from old redis RDB, error while serde {}, data received: {}",
                        challenge.err().unwrap(),
                        data
                    );
                }
                challenge.unwrap()
            }
            _ => panic!("Can't load mCaptcha from old redis RDB, encver {}", encver),
        };

        Box::into_raw(Box::new(challenge)) as *mut c_void
    }

    pub unsafe extern "C" fn free(value: *mut c_void) {
        let val = value as *mut Challenge;
        Box::from_raw(val);
    }

    #[allow(non_snake_case, unused)]
    pub unsafe extern "C" fn rdb_save(rdb: *mut raw::RedisModuleIO, value: *mut c_void) {
        let challenge = &*(value as *mut Challenge);
        match &serde_json::to_string(challenge) {
            Ok(string) => raw::save_string(rdb, &string),
            Err(e) => panic!("error while rdb_save: {}", e),
        }
    }
}
