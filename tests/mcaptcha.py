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

MCAPTCHA = {
  "visitor_threshold": 0,
  "defense": {
    "levels": [
      {"visitor_threshold": 50, "difficulty_factor": 50},
      {"visitor_threshold": 500, "difficulty_factor": 500}
    ],
    "current_visitor_threshold": 0
  },
  "duration": 5
}

COMMANDS = {
    "ADD_CAPTCHA": "MCAPTCHA_CACHE.ADD_CAPTCHA",
}

payload = json.dumps(MCAPTCHA)

def register(r, key):
    if r.exists(key):
        r.delete(key)

    r.execute_command(COMMANDS["ADD_CAPTCHA"], key, payload)
