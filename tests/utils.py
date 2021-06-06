
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

from redis.client import Redis
from redis import BlockingConnectionPool

"""Connect to redis instance"""
def connect(url):
    r = Redis(connection_pool=BlockingConnectionPool(max_connections=4))
    r.from_url(url)
    return r

"""Ping Redis Instance"""
def ping(r):
    resp = r.ping()
    assert resp is True
