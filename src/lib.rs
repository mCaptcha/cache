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
use lazy_static::lazy_static;
use redis_module::NotifyEvent;
use redis_module::{redis_command, redis_event_handler, redis_module};
use redis_module::{NextArg, RedisResult};
//use redis_module::RedisError;
use redis_module::Context;

mod bucket;
mod errors;
mod mcaptcha;
#[allow(dead_code, unused_features, unused_variables)]
mod safety;
mod utils;

use bucket::MCAPTCHA_BUCKET_TYPE;
use mcaptcha::MCAPTCHA_MCAPTCHA_TYPE;
use safety::MCAPTCHA_SAFETY_TYPE;

/// Initial allocation ammount of bucket[bucket::Bucket]
pub const HIT_PER_SECOND: usize = 100;

pub const PKG_NAME: &str = "mcap";
pub const PKG_VERSION: usize = 0;

/// bucket timer key prefix
// PREFIX_BUCKET_TIMER is used like this:
// PREFIX_BUCKET_TIMER:PREFIX_BUCKET:time(where time is variable)
// It contains PKG_NAME and key hash tag for node pinning
// so, I guess it's okay for us to just use timer and not enfore pinning
// and PKG_NAME
pub const PREFIX_BUCKET_TIMER: &str = "timer:";
pub const PREFIX_SAFETY: &str = "safety:";

/// If buckets perform clean up at x instant, then buckets themselves will get cleaned
/// up at x + BUCKET_EXPIRY_OFFSET(if they haven't already been cleaned up)
pub const BUCKET_EXPIRY_OFFSET: u64 = 30;

lazy_static! {


    /// node unique identifier, useful when running in cluster mode
    pub static ref ID: usize = {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        rng.gen()
    };
    /// counter/captcha key prefix
    pub static ref PREFIX_CAPTCHA: String = format!("{}:captcha::", PKG_NAME);
    /// bucket key prefix
    pub static ref PREFIX_BUCKET: String = format!("{}:bucket:{{{}}}:", PKG_NAME, *ID);
}

pub fn on_delete(ctx: &Context, event_type: NotifyEvent, event: &str, key_name: &str) {
    let msg = format!(
        "Received event: {:?} on key: {} via event: {}",
        event_type, key_name, event
    );
    ctx.log_debug(msg.as_str());

    if utils::is_bucket_timer(key_name) {
        bucket::Bucket::on_delete(ctx, event_type, event, key_name);
        return;
    } else if utils::is_mcaptcha_safety(key_name) {
        crate::safety::MCaptchaSafety::on_delete(ctx, event_type, event, key_name);
    }
}

redis_module! {
    name: "mcaptcha_cahce",
    version: PKG_VERSION,
    data_types: [MCAPTCHA_BUCKET_TYPE, MCAPTCHA_MCAPTCHA_TYPE, MCAPTCHA_SAFETY_TYPE],
    commands: [
        ["mcaptcha_cache.add_visitor", bucket::Bucket::counter_create, "write", 1, 1, 1],
        ["mcaptcha_cache.get", mcaptcha::MCaptcha::get_count, "readonly", 1, 1, 1],
        ["mcaptcha_cache.add_captcha", mcaptcha::MCaptcha::add_captcha, "readonly", 1, 1, 1],
        ["mcaptcha_cache.delete_captcha", mcaptcha::MCaptcha::delete_captcha, "write", 1, 1, 1],
        ["mcaptcha_cache.captcha_exists", mcaptcha::MCaptcha::captcha_exists, "readonly", 1, 1, 1],
    ],
   event_handlers: [
        [@EXPIRED @EVICTED: on_delete],
        //TODO add expire/evicted event for safety
    ]
}
