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
use redis_module::{redis_command, redis_module};
use redis_module::{Context, NextArg, RedisResult};

mod pocket;
mod utils;

use pocket::MCAPTCHA_POCKET_TYPE;

fn timer_create(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    // mcaptcha captcha key name
    let key_name = args.next_string()?;
    // expiry
    let duration = args.next_u64()?;
    pocket::Pocket::increment(ctx, duration, &key_name)?;

    Ok("OK".into())
}

//////////////////////////////////////////////////////

redis_module! {
    name: "mcaptcha_cahce",
    version: 1,
    data_types: [MCAPTCHA_POCKET_TYPE,],
    commands: [
        ["mcaptcha_cache.create", timer_create, "write", 1, 2, 1],
    ],
}
