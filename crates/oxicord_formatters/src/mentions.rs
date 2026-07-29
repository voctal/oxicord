//! Mentions formatters.

/// Formats a user mention `<@user_id>`.
///
/// See the [Discord Message Formatting](https://docs.discord.com/developers/reference#message-formatting) docs.
pub fn user_mention(user_id: u64) -> String {
    format!("<@{user_id}>")
}

/// Formats a channel mention `<#channel_id>`.
///
/// See the [Discord Message Formatting](https://docs.discord.com/developers/reference#message-formatting) docs.
pub fn channel_mention(channel_id: u64) -> String {
    format!("<#{channel_id}>")
}

/// Formats a role mention `<@&role_id>`.
///
/// See the [Discord Message Formatting](https://docs.discord.com/developers/reference#message-formatting) docs.
pub fn role_mention(role_id: u64) -> String {
    format!("<@&{role_id}>")
}

/// Formats a slash command mention `</name:id>`.
///
/// Use [`slash_subcommand_mention`] for subcommands mentions.
/// Use [`slash_subcommand_group_mention`] for subcommands groups mentions.
///
/// See the [Discord Message Formatting](https://docs.discord.com/developers/reference#message-formatting) docs.
pub fn slash_command_mention(name: &str, command_id: u64) -> String {
    format!("</{name}:{command_id}>")
}

/// Formats a slash command mention `</name subcommand:id>`.
///
/// Use [`slash_command_mention`] for root slash commands mentions.
/// Use [`slash_subcommand_group_mention`] for subcommands groups mentions.
///
/// See the [Discord Message Formatting](https://docs.discord.com/developers/reference#message-formatting) docs.
pub fn slash_subcommand_mention(name: &str, subcommand: &str, command_id: u64) -> String {
    format!("</{name} {subcommand}:{command_id}>")
}

/// Formats a slash command mention `</name group subcommand:id>`.
///
/// Use [`slash_command_mention`] for root slash commands mentions.
/// Use [`slash_subcommand_mention`] for subcommands mentions.
///
/// See the [Discord Message Formatting](https://docs.discord.com/developers/reference#message-formatting) docs.
pub fn slash_subcommand_group_mention(
    name: &str,
    group: &str,
    subcommand: &str,
    command_id: u64,
) -> String {
    format!("</{name} {group} {subcommand}:{command_id}>")
}

/// Formats custom emojis `<:name:id>` (static) or `<a:name:id>` (animated).
///
/// See the [Discord Message Formatting](https://docs.discord.com/developers/reference#message-formatting) docs.
pub fn format_emoji(name: &str, id: u64, animated: bool) -> String {
    if animated {
        format!("<a:{name}:{id}>")
    } else {
        format!("<:{name}:{id}>")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mention_formatters() {
        assert_eq!(user_mention(123), "<@123>");
        assert_eq!(channel_mention(123), "<#123>");
        assert_eq!(role_mention(123), "<@&123>");
        assert_eq!(slash_command_mention("help", 123), "</help:123>");
        assert_eq!(
            slash_subcommand_mention("info", "user", 123),
            "</info user:123>"
        );
        assert_eq!(
            slash_subcommand_group_mention("name", "group", "sub", 123),
            "</name group sub:123>"
        );
    }
}
