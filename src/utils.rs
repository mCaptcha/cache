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
use std::time::{SystemTime, UNIX_EPOCH};

use redis_module::RedisError;

pub const PREFIX_COUNTER: &str = "mcaptcha_cache:captcha:";
pub const PREFIX_TIME_POCKET: &str = "mcaptcha_cache:pocket:";

#[inline]
/// duration in seconds
pub fn get_pocket_name(pocket_instant: u64) -> String {
    format!("{}{}", PREFIX_TIME_POCKET, pocket_instant)
}

#[inline]
pub fn pocket_instant(duration: u64) -> Result<u64, RedisError> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(val) => Ok(val.as_secs() + duration),
        Err(_) => Err(RedisError::String("SystemTime before UNIX EPOCH!".into())),
    }
}

#[inline]
pub fn get_captcha_key(name: &str) -> String {
    format!("{}{}", PREFIX_COUNTER, name)
}
