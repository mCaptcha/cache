<div align="center">
  <h1>mCaptcha Cache</h1>
  <p>
    <strong>
      Redis module that implements
      <a href="https://en.wikipedia.org/wiki/Leaky_bucket"
        >leaky bucket algorithm</a
      >
    </strong>
  </p>

[![dependency status](https://deps.rs/repo/github/mCaptcha/cache/status.svg)](https://deps.rs/repo/github/mCaptcha/cache)
[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg?style=flat-square)](http://www.gnu.org/licenses/agpl-3.0)
[![Chat](https://img.shields.io/badge/matrix-+mcaptcha:matrix.batsense.net-purple?style=flat-square)](https://matrix.to/#/+mcaptcha:matrix.batsense.net)

</div>

## Motivation

[mCaptcha](https://github.com/mCaptcha/mCaptcha) uses a [leaky-
bucket](https://en.wikipedia.org/wiki/Leaky_bucket)-enabled counter to
keep track of traffic/challenge requests.

- At `t=0`(where `t` is time), if someone is visiting an mCaptcha-protected website, the
  counter for that website will be initialized and set to 1.

- It should also automatically decrement(by 1) after a certain period, say
  `t=cooldown`. We call this cool down period and is constant for a
  website.

- If at `t=x`(where `x<cooldown`), another user visits the same website,
  the counter becomes 2 and will auto decrement at `t = cooldown + x`
  for second user.

  Note that, for the decrement to work, we require two different timers
  that goes off at two different instants. The current(`v0.1.3`) of
  [`libmcaptcha`](https://github.com/mCaptcha/libmcaptcha/) implements
  this with internal data structures and timers --- something that can't
  be shared across several machines in a distributed setting.

  So we figured we'd use Redis to solve this problem and get
  synchronisation and persistence for free.

  This Redis module implements auto decrement on a special
  data type(which is also defined in this module).

## How does it work?

If a timer is supposed to go off to
decrement key `myCounter` at `t=y`(where y is an instant in future),

1. A hashmap called `mcaptcha_cache:decrement:y`(prefix might vary) is
   created with key-value pairs `keyName: DecrementCount`(`myCounter: 1` in
   our case)

2. A timer will be created to go off at `t=y`
3. Any further decrement operations that are scheduled for `t=y` are
   registered with the same hashmap(`mcaptcha_cache:decrement:y`).

4. At `t=y`, a procedure will be executed to read
   all values of the hashmap(`mcaptcha_cache:decrement:y`) and performs
   all registered decrements. When its done, it cleans itself up.

This way, we are not spinning timers for every decrement operation but
instead, one for every "time pocket".

### Gotchas:

This module creates and manages data of two types:

1.  `mcaptcha_cache:captcha:y` where `y`(last character) is variable
2.  `mcaptcha_cache:pocket:x` where `x`(last character) is variable

**WARNING: Please don't modify these manually. If you do so, then Redis
will panic**

This module is capable of cleaning up after itself so manual clean up is
unnecessary. If you have needs that are not met my this module and you
which access/mutate data manually, please open an
[issue](https://github.com/mCaptcha/cache/issues). I'd be happy to help.

## Usage

There are two ways to run `cache`:

1. [Using docker](#docker)
2. [On bare-metal](#bare-metal)

### Docker

Use image from DockerHub:

```bash
$  docker run -p 6379:6379 mcaptcha/cache:0.1.1-alpha
```

or build from source:

#### Build

```bash
$ docker build -t mcaptcha/cache:latest .
```

#### Run

```bash
$  docker run -p 6379:6379 mcaptcha/cache:latest
```

### Bare-metal

#### Build

Make sure you have Rust installed:
https://www.rust-lang.org/tools/install

Then, build as usual:

```bash
cargo build --release
```

#### Run

```
redis-server --loadmodule ./target/release/libcache.so
```

### Commands

Every counter has a name and a leak-rate in seconds.

## Create/Increment counter

If counter exists, then count is incremented. Otherwise, it is created.

```redis
MCAPTCHA_CACHE.COUNT <counter-name> <leak-rate>
```

## Benchmark

**NOTE:** These benchmarks are for reference only. Do not depend upon
them too much. When in doubt, please craft and run benchmarks that are
better suited to your workload.

- platform: `Intel core i7-9750h`

### With request pipelining

```bash
➜  ~ redis-benchmark -n 1000000 -t set,get -P 16 -q # set and get are for baseline/reference
SET: 835421.88 requests per second, p50=0.759 msec
GET: 987166.81 requests per second, p50=0.711 msec

➜  ~ redis-benchmark -n 1000000 -P 16 -q MCAPTCHA_CACHE.COUNT mycounter 45
MCAPTCHA_CACHE.COUNT mycounter 45: 280504.91 requests per second, p50=2.743 msec
```

### Without request pipelining

```bash
➜  ~ redis-benchmark -n 1000000 -t set,get -q # set and get are for baseline/reference
SET: 87062.51 requests per second, p50=0.311 msec
GET: 87252.41 requests per second, p50=0.311 msec


➜  ~ redis-benchmark -n 1000000 -q MCAPTCHA_CACHE.COUNT mycounter 45
MCAPTCHA_CACHE.COUNT mycounter 45: 87214.38 requests per second, p50=0.471 msec
```
