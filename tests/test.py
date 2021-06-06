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

from time import sleep

from redis.client import Redis
from redis import BlockingConnectionPool

import utils
from runner import Runner
import bucket

REDIS_URL = "redis://localhost:6350"


r = utils.connect(REDIS_URL)
utils.ping(r)


def main():
    #runner = Runner()
    #fn = [bucket.incr_one_works]#, bucket.race_works]

    bucket.incr_one_works()
    bucket.race_works()

    #try:
    #    for r in fn:
    #        runner.register(r)

    #    runner.wait()
    #    print("All tests passed")
    #except Exception as e:
    #    raise e

if __name__ == "__main__":
    main()
