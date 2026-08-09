<div align="center">
    <img src="https://github.com/voctal/oxicord/raw/HEAD/crates/oxicord/docs/images/ferris.png" width="152" alt="Ferris">
    <h1>oxicord_cdn</h1>
    <p>
        <a href="https://voctal.dev/discord"><img src="https://img.shields.io/discord/1336303640725553213?color=5865F2&logo=discord&logoColor=white" alt="Discord server" /></a>
        <a href="https://github.com/voctal/oxicord/commits/main"><img alt="Last commit" src="https://img.shields.io/github/last-commit/voctal/oxicord?logo=github&logoColor=ffffff" /></a>
    </p>
</div>

## oxicord_cdn

Discord CDN utilities.

See <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>

The crate contains:

- A `Cdn` struct with link builders
- A `calculate_user_default_avatar_index` utility
- Constants (cdn url, allowed images extensions, etc.)

## TODO

- Make a CdnRoutes static builders struct in api_types
- Add a `StickerPackApplicationId = 710982414301790216` constant in api_types, and import it here
