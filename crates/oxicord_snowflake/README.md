<div align="center">
    <img src="https://github.com/voctal/oxicord/raw/HEAD/crates/oxicord/docs/images/ferris.png" width="152" alt="Ferris">
    <h1>oxicord_snowflake</h1>
    <p>
        <a href="https://voctal.dev/discord"><img src="https://img.shields.io/discord/1336303640725553213?color=5865F2&logo=discord&logoColor=white" alt="Discord server" /></a>
        <a href="https://github.com/voctal/oxicord/commits/main"><img alt="Last commit" src="https://img.shields.io/github/last-commit/voctal/oxicord?logo=github&logoColor=ffffff" /></a>
    </p>
</div>

## About

Snowflake implementation and utilities.

The crate exports an `Id` struct which represents a Snowflake.
It takes a generic which is purely a compile-time safety, to prevent
using IDs of a certain type to a function asking for another.

```rust
use oxicord_snowflake::{Id, UserMarker, ChannelMarker};

fn something(id: Id<RoleMarker>) { ... };

let guild_id: Id<GuildMarker> = ...;

// doing `something(guild_id)` would cause a compile time error
// since the marker are not equals. However, in that case,
// we know that the everyone role id is the same as the guild id.
// so we can cast it (changes the T in Id<T> to another one).
// This is purely compile-time and casting has no effect at runtime.
let everyone_id: Id<RoleMarker> = guild_id.cast();
something(everyone_id);
```

However in practice the crate already exports shorthand for markers:

```rust
// UserId is `type UserId = Id<UserMarker>`
use oxicord_snowflake::UserId;

// You can use this:
let user_id: UserId;
// Instead of that:
let user_id: Id<UserMarker>;
```

If you have an ID in `u64`, use `Id::from_raw` to get the `Id`.
On an `Id`, use `.get` to get the inner `u64` value.

## Timestamps

The crate also exports utilities to:

- extract the timestamp of a snowflake (if you already
  have the `Id` instance, you can call `.timestamp()` instead)
- create a snowflake from a timestamp, e.g. for pagination query params
