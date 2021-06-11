#!/bin/env /usr/bin/python3
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
from asyncio import sleep
import json

import redis

import utils

r = utils.connect()
utils.ping(r)

# 1. Check duplicate challenge
# 2. Create challenge
# 3. Read non-existent challenge
# 4. Read challenge
# 5. Read expired challenge


COMMANDS = {
 "ADD" :"MCAPTCHA_CACHE.ADD_CHALLENGE",
 "GET" :"MCAPTCHA_CACHE.GET_CHALLENGE",
 "DEL" :"MCAPTCHA_CACHE.DELETE_CHALLENGE"
}

CHALLENGE_NOT_FOUND = "Challenge not found"
DUPLICATE_CHALLENGE = "Challenge already exists"
REDIS_OK = bytes("OK", 'utf-8')

def add_challenge(captcha, challenge):
    """Add challenge to Redis"""
    try :
        return r.execute_command(COMMANDS["ADD"], captcha, challenge)
    except Exception as e:
        return e
    
def get_challenge_from_redis(captcha, challenge):
    """Add challenge to Redis"""
    try :
        data = r.execute_command(COMMANDS["GET"], captcha, challenge)
        return json.loads(data)
    except Exception as e:
        return e

def delete_challenge(captcha, challenge):
    """Add challenge to Redis"""
    try :
        data = r.execute_command(COMMANDS["DEL"], captcha, challenge)
        return data
    except Exception as e:
        return e


def get_challenge(challenge):
    """Get challenge JSON"""
    challenge = {
        "difficulty": 500,
        "duration": 5,
        "challenge": challenge,
    }
    return json.dumps(challenge)


async def add_challenge_works():
    """Test: Add Challenge"""
    try:
        key = "add_challenge"
        challenge_name = "add_challenge_challenge"
        challenge = get_challenge(challenge_name)

        add_challenge(key, challenge)
        stored_challenge = get_challenge_from_redis(key, challenge_name)
        challenge_dict = json.loads(challenge)
        assert stored_challenge["difficulty_factor"] == challenge_dict["difficulty"]
        assert stored_challenge["duration"] == challenge_dict["duration"]
        error = get_challenge_from_redis(key, challenge_name)
        assert str(error) == CHALLENGE_NOT_FOUND
        print("[*] Add Challenge works")

    except Exception as e:
        raise e

async def challenge_ttl_works():
    """Test: Challenge TTL"""
    try:
        key = "ttl_challenge"
        challenge_name = "ttl_challenge_challenge"
        challenge = get_challenge(challenge_name)

        add_challenge(key, challenge)
        await sleep(5 + 2)

        error = get_challenge_from_redis(key, challenge_name)
        assert str(error) == CHALLENGE_NOT_FOUND

        print("[*] Challenge TTL works")
    except Exception as e:
        raise e


async def challenge_doesnt_exist():
    """Test: Non-existent Challenge"""
    try:
        challenge_name = "nonexistent_challenge"
        key = "nonexistent_challenge_key"

        error = get_challenge_from_redis(key, challenge_name)
        assert str(error) == CHALLENGE_NOT_FOUND

        print("[*] Challenge Doesn't Exist works")
    except Exception as e:
        raise e


async def duplicate_challenge_works():
    """Test: Duplicate Challenges"""
    try:
        challenge_name = "nonexistent_challenge"
        key  = challenge_name
        challenge = get_challenge(challenge_name)

        add_challenge(key, challenge)
        error = add_challenge(key, challenge)
        assert str(error) == DUPLICATE_CHALLENGE

        print("[*] Duplicate Challenge works")
    except Exception as e:
        raise e

async def delete_challenge_works():
    """Test: Delete Challenges"""
    try:
        challenge_name = "delete_challenge"
        key = "delete_challenge_key"
        challenge = get_challenge(challenge_name)

        add_challenge(key, challenge)
        resp = delete_challenge(key, challenge_name)
        assert resp == REDIS_OK
        resp = delete_challenge(key, challenge_name)
        assert str(resp) == CHALLENGE_NOT_FOUND

        print("[*] Delete Challenge works")
    except Exception as e:
        raise e
