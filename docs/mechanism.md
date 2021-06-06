# mCaptcha Redis Module Mechanism

## Data types

1. Bucket
2. Bucket safety
3. mCaptcha
4. mCaptcha safety

## Bucket

- Timer queue, used for scheduling decrements.
- Timer used for scheduling can't be persisted so requires a safety

## Bucket Safety

- Has expiry timer(Redis `EXIPRE`)
- Key name includes bucket name for which this safety was created for
- When Redis is started after crash, bucket safety expires and expire
  event is fired, the corresponding bucket is fetched and executed.

## mCaptcha

- Contains mCaptcha defense details and current state
- When redis performs resharding, it's possible that mCaptcha gets
  separated from its bucket.
- We can't pin mCaptcha by using hashtags because the client figures out
  where a key should be placed/is available and will have no
  knowledge about node IDs that we use for pinning.

- So this too requires a safety to make sure that when recovering from a
  crash, it's counter doesn't have residues permanently.

## mCaptcha safety

- Has expire timer(Redis `EXIPRE`)
- Key name includes mCaptcha name for which this safety was created for
- when expiry event is fired for this type, a bucket for corresponding
  mCaptcha is created to decrement it by `x`(where `x` is the count of mCaptcha
  when this event is fired). It's not perfect but at least we get
  eventual consistency.
