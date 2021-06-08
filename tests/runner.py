#!/bin/env /usr/bin/python3
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
from threading import Thread
import asyncio

import bucket
import mcaptcha


class Runner(object):
    __fn = [
        bucket.incr_one_works,
        bucket.race_works,
        bucket.difficulty_works,
        mcaptcha.delete_captcha_works,
        mcaptcha.captcha_exists_works,
        mcaptcha.register_captcha_works
    ]
    __tasks = []

    async def __register(self):
        """ Register functions to be run"""
        for fn in self.__fn:
            task = asyncio.create_task(fn())
            self.__tasks.append(task)


    async def run(self):
        """Wait for registered functions to finish executing"""
        await self.__register()
        for task in self.__tasks:
            await task

    """Runs in seperate threads"""
    def __init__(self):
        super(Runner, self).__init__()
