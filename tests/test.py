#!/bin/env python3 

from time import sleep

from redis.client import Redis
from redis import BlockingConnectionPool

REDIS_URL = "redis://localhost:6350"

COMMANDS = {
"COUNT" : "mcaptcha_cache.count",
"GET" : "mcaptcha_cache.get",
}


KEY = "testing_module"
TIME = 20

def connect():
    r = Redis(connection_pool=BlockingConnectionPool(max_connections=2))
    r.from_url(REDIS_URL)
    return r

r = connect()

def ping():
    resp = r.ping()
    assert resp is True

def incr(key, time):
    r.execute_command(COMMANDS["COUNT"], key, time)

def get_count(key):
    try:
        count = r.execute_command(COMMANDS["GET"], key)
        return int(count)
    except:
        return 0

def race(count):
    for _ in range(count):
        incr(KEY, TIME)

def assert_count(expect):
    count = get_count(KEY)
    assert count == expect

def main():
    # check connectivity
    ping()
    # get initial count(residual)
    initial_count = get_count(KEY)
    # incriment
    incr(KEY, TIME)
    assert_count(initial_count + 1)
    # wait till expiry
    sleep(TIME + 4)
    assert_count(initial_count)
    # increment by 200
    race_num = 200
    race(race_num)
    assert_count(initial_count + race_num)
    # wait till expiry
    sleep(TIME + 4)
    assert_count(initial_count)
    print("All good")


if __name__ == "__main__":
    main()
