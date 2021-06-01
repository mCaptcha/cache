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

[![dependency status](https://deps.rs/repo/github/mCaptcha/cache/status.svg?style=flat-square)](https://deps.rs/repo/github/mCaptcha/cache)
[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg?style=flat-square)](http://www.gnu.org/licenses/agpl-3.0)
[![Chat](https://img.shields.io/badge/matrix-+mcaptcha:matrix.batsense.net-purple?style=flat-square)](https://matrix.to/#/+mcaptcha:matrix.batsense.net)

</div>

## Motivation

[mCaptcha](https://github.com/mCaptcha/mCaptcha) uses a [leaky-
bucket](https://en.wikipedia.org/wiki/Leaky_bucket)-enabled counter to
keep track of traffic/challenge requests.

- At `t=0`, if someone is visiting an mCaptcha-protected website, the
  counter for that website will be initialized and set to 1.

- It should also automatically decrement(by 1) after a certain period, say
  `t=cooldown`. We call this cool down period and is constant for a
  website.

- If at `t=x`(where `x<cooldown`), another user visits the same website,
  the counter becomes 2 and will auto decrement at `t = cooldown + x`.

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
