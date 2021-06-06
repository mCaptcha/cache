#!/bin/env python3 
#
# Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
# 
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.
# 
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
# 
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.
from time import sleep

import test;

r = test.r

COMMANDS = {
"COUNT" : "mcaptcha_cache.count",
"GET" : "mcaptcha_cache.get",
}

def incr(key, time):
    r.execute_command(COMMANDS["COUNT"], key, time)

def get_count(key):
    try:
        count = r.execute_command(COMMANDS["GET"], key)
        return int(count)
    except:
        return 0

def assert_count(expect, key):
    count = get_count(key)
    assert count == expect

def incr_one_works():
    key = "incr_one"
    time = 2
    initial_count = get_count(key)
    # incriment
    incr(key, time)
    assert_count(initial_count + 1, key)
    # wait till expiry
    sleep(time + 2)
    assert_count(initial_count, key)
    print("Incr one works")

def race_works():
    key = "race_works"
    initial_count = get_count(key)
    race_num = 200
    time = 3

    for _ in range(race_num):
        incr(key, time)
    assert_count(initial_count + race_num, key)
    # wait till expiry
    sleep(time + 2)
    assert_count(initial_count, key)
    print("Race works")
