#!/bin/sh
readonly NUM_REQUESTS=1000000

readonly CAPTCHA_NAME=mycounter
readonly DURATION=45

scripts/setupbench.py $CAPTCHA_NAME $DURATION

echo "running set and get without pipelining" 
redis-benchmark -n 1000000 -t set,get -q

echo "mCaptcha cache without piplining"
redis-benchmark -n $NUM_REQUESTS -q MCAPTCHA_CACHE.ADD_VISITOR $CAPTCHA_NAME
redis-benchmark -n $NUM_REQUESTS -q MCAPTCHA_CACHE.GET $CAPTCHA_NAME

echo "running set and get with pipelining" 
redis-benchmark -n 1000000 -t set,get -q -P 16

echo "mCaptcha cache with piplining"
redis-benchmark -P 16 -n $NUM_REQUESTS -q MCAPTCHA_CACHE.ADD_VISITOR $CAPTCHA_NAME
redis-benchmark -P 16 -n $NUM_REQUESTS -q MCAPTCHA_CACHE.GET $CAPTCHA_NAME
