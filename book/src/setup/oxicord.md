# Installation (Oxicord)

Oxicord is divided in many specialized sub-crates. If you're planning to make
a bot, you probably need those to start:

```sh
cargo add oxicord oxicord_api_types oxicord_builders oxicord_cache
```

We highly recommend you to read the READMEs of our other crates to know
what are their purpose, and know which one you need. The "common" ones are:

- `oxicord_builders` - Builders for components, embeds, etc.
- `oxicord_formatters` - Formatters for markdown or mentions
- `oxicord_patterns` - Regexes for Discord
- `oxicord_api_types` - All types from the API
