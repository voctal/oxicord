<div align="center">
    <img src="https://github.com/voctal/oxicord/raw/HEAD/crates/oxicord/docs/images/ferris.png" width="152" alt="Ferris">
    <h1>Oxicord</h1>
    <p>
        <a href="https://voctal.dev/discord"><img src="https://img.shields.io/discord/1336303640725553213?color=5865F2&logo=discord&logoColor=white" alt="Discord server" /></a>
        <a href="https://github.com/voctal/oxicord/commits/main"><img alt="Last commit" src="https://img.shields.io/github/last-commit/voctal/oxicord?logo=github&logoColor=ffffff" /></a>
    </p>
</div>

## About

Oxicord is a collection of Rust libraries to interact with the Discord API.

The library is made to be performant, highly scalable, maintainable, extensible, fully documented, and easy to use.

The REST API v10+ and the gateway will be supported, but not the voice API (at least for now).

> [!IMPORTANT]
> `oxicord` is in development.

## Crates

- `oxicord` - The main crate
- `oxicord_api_types` - The Discord API types
- `oxicord_builders` - Builders for Discord structures (components, embeds, ...)
- `oxicord_cache` - Cache implementations (in-memory, Redis...)
- `oxicord_core` - Core library (e.g. for HTTP-only bots)
- `oxicord_rest` - REST http for the API
- `oxicord_formatters` - Formatting utilities for messages
- `oxicord_structures` - Wrappers for Discord structures
- `oxicord_utils` - Utilities for Oxicord
- `oxicord_macros` - Macros for Oxicord
- `oxicord_gateway` - Gateway implementation (WebSocket)
- `oxicord_collector` - Collectors
- `oxicord_pagination` - Pagination helper
- `oxicord_framework` - Complex framework around Oxicord

## TODO

- rustls
- `features`
- MSRV 1.96?
- ratelimits (error/waiting configurable by endpoint)
