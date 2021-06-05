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

use crate::*;

#[inline]
/// duration in seconds
pub fn get_pocket_name(pocket_instant: u64) -> String {
    format!("{}{}", &*PREFIX_POCKET, pocket_instant)
}

#[inline]
/// duration in seconds
pub fn get_timer_name_from_pocket_name(pocket_name: &str) -> String {
    format!("{}{}", &*PREFIX_POCKET_TIMER, pocket_name)
}

#[inline]
/// duration in seconds
pub fn get_pocket_name_from_timer_name(name: &str) -> Option<&str> {
    // PREFIX_POCKET_TIMER doesn't have node unique crate::ID
    // this way, even if we are loading keys of a different instance, well
    // get POCKET keys from whatever TIMER is expiring
    name.strip_prefix(&*PREFIX_POCKET_TIMER)
}

#[inline]
pub fn get_pocket_instant(duration: u64) -> Result<u64, RedisError> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(val) => Ok(val.as_secs() + duration),
        Err(_) => Err(RedisError::String("SystemTime before UNIX EPOCH!".into())),
    }
}

#[inline]
pub fn get_captcha_key(name: &str) -> String {
    format!("{}{}", &*PREFIX_COUNTER, name)
}

#[inline]
pub fn is_pocket_timer(name: &str) -> bool {
    name.contains(&*PREFIX_POCKET_TIMER)
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn timer_name_works() {
//        const POCKET_INSTANT: u64 = 12345678;
//        let pocket_name: String = get_pocket_name(POCKET_INSTANT);
//
//        let timer_name = get_timer_name_from_pocket_name(&pocket_name);
//        assert_eq!(
//            get_pocket_name_from_timer_name(&timer_name),
//            Some(pocket_name.as_str())
//        );
//    }
//}
