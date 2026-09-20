<div align="center">
    <img src="https://github.com/voctal/oxicord/raw/HEAD/crates/oxicord/docs/images/ferris.png" width="152" alt="Ferris">
    <h1>oxicord_rest</h1>
    <p>
        <a href="https://voctal.dev/discord"><img src="https://img.shields.io/discord/1336303640725553213?color=5865F2&logo=discord&logoColor=white" alt="Discord server" /></a>
        <a href="https://github.com/voctal/oxicord/commits/main"><img alt="Last commit" src="https://img.shields.io/github/last-commit/voctal/oxicord?logo=github&logoColor=ffffff" /></a>
    </p>
</div>

## oxicord_rest

Library to interact with Discord REST API and CDN.
Uses [hyper](https://crates.io/crates/hyper) under the hood.

See `oxicord_cdn` for the CDN types and constants.

## Features

- [ ] Ratelimits
    - [ ] Global / Buckets
    - [ ] Sublimits
    - [ ] In-memory ratelimiter
    - [ ] Redis ratelimiter
- [ ] Concurrency
- [ ] Events
- [ ] Files
- [ ] SIMD

## How does it work (TODO)

- explain global = 50req/s or more
- explain "bucket" (and the hashing)
- explain burst vs sequential
- explain sublimits and how to register them and why
- explain that you need execute_webhook to prevent sequential from running

## TODO

- Routes helpers
- Ratelimiter
- A LOT of tests
- Sequential / Burst
- simd
- async redis ratelimiter
