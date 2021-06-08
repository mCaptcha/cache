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

import json

import utils

r = utils.connect()
utils.ping(r)

MCAPTCHA = {
  "levels": [
      {"visitor_threshold": 50, "difficulty_factor": 50},
      {"visitor_threshold": 500, "difficulty_factor": 500}
    ],
  "duration": 5
}

COMMANDS = {
    "ADD_CAPTCHA": "MCAPTCHA_CACHE.ADD_CAPTCHA",
    "DELETE_CAPTCHA": "MCAPTCHA_CACHE.DELETE_CAPTCHA",
    "CAPTCHA_EXISTS": "MCAPTCHA_CACHE.CAPTCHA_EXISTS",
}

payload = json.dumps(MCAPTCHA)

def delete_captcha(key):
    r.execute_command(COMMANDS["DELETE_CAPTCHA"], key)


def add_captcha(key):
    r.execute_command(COMMANDS["ADD_CAPTCHA"], key, payload)


def captcha_exists(key):
    exists = r.execute_command(COMMANDS["CAPTCHA_EXISTS"], key)
    if exists == 0:
        return True

    if exists == 1:
        return False

def register(key):
    if captcha_exists(key):
        delete_captcha(key)

    add_captcha(key)

async def captcha_exists_works():
    key = "captcha_delete_works"
    if captcha_exists(key):
        delete_captcha(key)
    assert captcha_exists(key) is False
    register(key)
    assert captcha_exists(key) is True
    print("[*] Captcha delete works")

async def register_captcha_works():
    key = "register_captcha_works"
    register(key)
    assert captcha_exists(key) is True
    print("[*] Add captcha works")

async def delete_captcha_works():
    key = "delete_captcha_works"
    register(key)
    exists = captcha_exists(key)
    assert exists is True
    delete_captcha(key)
    assert captcha_exists(key) is False
    print("[*] Delete captcha works")
