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

class Runner(object):
    _functions = []
    _tasks = []

    async def register(self, fn):
        """ Register functions to be run"""
        self._functions.append(fn)
        task = asyncio.create_task(fn())
        self._tasks.append(task)


    async def wait(self):
        """Wait for registered functions to finish executing"""

        for task in self._tasks:
            await task

    """Runs in seperate threads"""
    def __init__(self):
        super(Runner, self).__init__()
