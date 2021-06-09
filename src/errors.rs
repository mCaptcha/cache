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

use std::num::ParseIntError;

use derive_more::Display;
use libmcaptcha::errors::CaptchaError;
use redis_module::RedisError;
use redis_module::RedisResult;

#[derive(Debug, Display)]
pub enum CacheError {
    #[display(fmt = "{}", &_0)]
    Msg(String),
    #[display(fmt = "{}", &_0.to_string)]
    RedisError(redis_module::RedisError),
    #[display(fmt = "Captcha not found")]
    CaptchaNotFound,
    #[display(fmt = "Challenge not found")]
    ChallengeNotFound,
    #[display(fmt = "Challenge already exists")]
    DuplicateChallenge,
}

impl CacheError {
    pub fn new(msg: String) -> Self {
        CacheError::Msg(msg)
    }
}

impl From<String> for CacheError {
    fn from(e: String) -> Self {
        CacheError::Msg(e.to_string())
    }
}

impl From<&str> for CacheError {
    fn from(e: &str) -> Self {
        CacheError::Msg(e.to_string())
    }
}

impl From<serde_json::Error> for CacheError {
    fn from(e: serde_json::Error) -> Self {
        CacheError::Msg(e.to_string())
    }
}

impl From<RedisError> for CacheError {
    fn from(e: redis_module::RedisError) -> Self {
        CacheError::RedisError(e)
    }
}

impl From<ParseIntError> for CacheError {
    fn from(e: ParseIntError) -> Self {
        let err: RedisError = e.into();
        CacheError::RedisError(err)
    }
}

impl From<CacheError> for RedisResult {
    fn from(e: CacheError) -> Self {
        Self::Err(e.into())
    }
}

impl From<CaptchaError> for CacheError {
    fn from(e: CaptchaError) -> Self {
        CacheError::Msg(format!("{}", e))
    }
}

impl From<CacheError> for RedisError {
    fn from(e: CacheError) -> Self {
        match e {
            CacheError::Msg(val) => RedisError::String(val),
            CacheError::RedisError(val) => val,
            CacheError::CaptchaNotFound => RedisError::String(format!("{}", e)),
            CacheError::ChallengeNotFound => RedisError::String(format!("{}", e)),
            CacheError::DuplicateChallenge => RedisError::String(format!("{}", e)),
        }
    }
}

pub type CacheResult<T> = Result<T, CacheError>;
