<div align="center">
    <img src="https://github.com/voctal/oxicord/raw/HEAD/crates/oxicord/docs/images/ferris.png" width="152" alt="Ferris">
    <h1>oxicord_patterns</h1>
    <p>
        <a href="https://voctal.dev/discord"><img src="https://img.shields.io/discord/1336303640725553213?color=5865F2&logo=discord&logoColor=white" alt="Discord server" /></a>
        <a href="https://github.com/voctal/oxicord/commits/main"><img alt="Last commit" src="https://img.shields.io/github/last-commit/voctal/oxicord?logo=github&logoColor=ffffff" /></a>
    </p>
</div>

## About

Regexes for Discord.

## Examples

Match a user mention and extract the ID:

```rust
use oxicord_patterns::USER_PATTERN;

let id = "123456789012345678";
let input = format!("<@{id}>");
let caps = USER_PATTERN.captures(&input).unwrap();

assert_eq!(&caps["id"], ID);
```
