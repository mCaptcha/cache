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

use redis_module::raw::KeyType;
use redis_module::{redis_command, redis_event_handler, redis_module};
use redis_module::{Context, NextArg, RedisResult, REDIS_OK};

mod errors;
mod pocket;
mod utils;

use pocket::MCAPTCHA_POCKET_TYPE;

/// Pocket[pocket::Pocket] type version
pub const REDIS_MCAPTCHA_POCKET_TYPE_VERSION: i32 = 0;

lazy_static! {
    pub static ref ID: usize = {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        rng.gen()
    };

    /// counter/captcha key prefix
    pub     static ref PREFIX_COUNTER: String = format!("mcaptcha_cache:captcha:");

    /// pocket key prefix
    pub     static ref PREFIX_POCKET: String = format!("mcaptcha_cache:pocket:{{{}}}:", *ID);
    /// pocket timer key prefix
    pub     static ref PREFIX_POCKET_TIMER: String = format!("mcaptcha_cache:timer:");


}

/// If pockets perform clean up at x instant, then pockets themselves will get cleaned
/// up at x + POCKET_EXPIRY_OFFSET(if they haven't already been cleaned up)
pub const POCKET_EXPIRY_OFFSET: u64 = 30;

fn counter_create(ctx: &Context, args: Vec<String>) -> RedisResult {
    counter_runner(ctx, args)
}

fn get(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key_name = args.next_string()?;
    let key_name = utils::get_captcha_key(&key_name);

    let stored_captcha = ctx.open_key(&key_name);
    if stored_captcha.key_type() == KeyType::Empty {
        return errors::CacheError::new(format!("key {} not found", key_name)).into();
    }

    Ok(stored_captcha.read()?.unwrap().into())
}

fn counter_runner(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    // mcaptcha captcha key name
    let key_name = args.next_string()?;

    // expiry
    let duration = args.next_u64()?;

    pocket::Pocket::increment(ctx, duration, &key_name)?;
    REDIS_OK
}

//////////////////////////////////////////////////////

redis_module! {
    name: "mcaptcha_cahce",
    version: 1,
    data_types: [MCAPTCHA_POCKET_TYPE,],
    commands: [
        ["mcaptcha_cache.count", counter_create, "write", 1, 1, 1],
        ["mcaptcha_cache.get", get, "readonly", 1, 1, 1],
    ],
   event_handlers: [
        [@EXPIRED @EVICTED: pocket::Pocket::on_delete],
    ]
}
