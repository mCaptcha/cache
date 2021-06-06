#!/bin/env python3 

from time import sleep
from threading import Thread

from redis.client import Redis
from redis import BlockingConnectionPool

REDIS_URL = "redis://localhost:6350"

COMMANDS = {
"COUNT" : "mcaptcha_cache.count",
"GET" : "mcaptcha_cache.get",
}

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


def main():
    # check connectivity
    ping()

    t1 = Thread(target=incr_one_works)
    t2 = Thread(target=race_works)

    t1.start()
    t2.start()
    t1.join()
    t2.join()

    print("All tests passed")


if __name__ == "__main__":
    main()
