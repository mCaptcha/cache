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

use redis_module::RedisError;
use redis_module::RedisResult;

#[derive(Debug)]
pub enum CacheError {
    Msg(String),
    RedisError(redis_module::RedisError),
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

impl From<redis_module::RedisError> for CacheError {
    fn from(e: redis_module::RedisError) -> Self {
        CacheError::RedisError(e)
    }
}

impl From<CacheError> for RedisError {
    fn from(e: CacheError) -> Self {
        match e {
            CacheError::Msg(val) => RedisError::String(val),
            CacheError::RedisError(val) => val,
        }
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
        match e {
            CacheError::Msg(val) => Err(RedisError::String(val)),
            CacheError::RedisError(val) => Err(val),
        }
    }
}

pub type CacheResult<T> = Result<T, CacheError>;
