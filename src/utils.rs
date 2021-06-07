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

use crate::errors::*;
use crate::*;

#[inline]
/// duration in seconds
pub fn get_bucket_name(bucket_instant: u64) -> String {
    format!("{}{}", &*PREFIX_BUCKET, bucket_instant)
}

#[inline]
/// duration in seconds
pub fn get_timer_name_from_bucket_name(bucket_name: &str) -> String {
    format!("{}{}", &*PREFIX_BUCKET_TIMER, bucket_name)
}

#[inline]
/// duration in seconds
pub fn get_bucket_name_from_timer_name(name: &str) -> Option<&str> {
    // PREFIX_BUCKET_TIMER doesn't have node unique crate::ID
    // this way, even if we are loading keys of a different instance, well
    // get BUCKET keys from whatever TIMER is expiring
    name.strip_prefix(&*PREFIX_BUCKET_TIMER)
}

#[inline]
pub fn get_bucket_instant(duration: u64) -> CacheResult<u64> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(val) => Ok(val.as_secs() + duration),
        Err(_) => Err(CacheError::new("SystemTime before UNIX EPOCH!".into())),
    }
}

#[inline]
pub fn get_captcha_key(name: &str) -> String {
    format!("{}{{{}}}", &*PREFIX_CAPTCHA, name)
}

#[inline]
pub fn get_safety_name(mcaptcha_name: &str) -> String {
    format!("{}{}", PREFIX_SAFETY, mcaptcha_name)
}

#[inline]
pub fn get_mcaptcha_from_safety(safety_name: &str) -> Option<&str> {
    safety_name.strip_prefix(&PREFIX_SAFETY)
}

#[inline]
pub fn is_bucket_timer(name: &str) -> bool {
    name.contains(&*PREFIX_BUCKET_TIMER)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timer_name_works() {
        const BUCKET_INSTANT: u64 = 12345678;
        let bucket_name: String = get_bucket_name(BUCKET_INSTANT);

        let timer_name = get_timer_name_from_bucket_name(&bucket_name);
        assert_eq!(
            get_bucket_name_from_timer_name(&timer_name),
            Some(bucket_name.as_str())
        );
    }
}
