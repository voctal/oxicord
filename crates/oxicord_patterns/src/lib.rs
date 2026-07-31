//! # oxicord_patterns

use regex::Regex;
use std::sync::LazyLock;

/// Regex for matching a user mention (strictly without a nickname).
///
/// Available groups:
/// - `id` - the user id.
pub static USER_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<@(?P<id>\d{17,20})>").unwrap());

/// Regex for matching a user mention with a nickname.
///
/// Available groups:
/// - `id` - the user id.
pub static USER_WITH_NICKNAME_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<@!(?P<id>\d{17,20})>").unwrap());

/// Regex for matching a user mention with or without a nickname.
///
/// Available groups:
/// - `id` - the user id.
pub static USER_WITH_OPTIONAL_NICKNAME_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<@!?(?P<id>\d{17,20})>").unwrap());

/// Regex for matching a channel mention.
///
/// Available groups:
/// - `id` - the channel id.
pub static CHANNEL_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<#(?P<id>\d{17,20})>").unwrap());

/// Regex for matching a role mention.
///
/// Available groups:
/// - `id` - the role id.
pub static ROLE_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<@&(?P<id>\d{17,20})>").unwrap());

/// Regex for matching an application command mention.
///
/// Available groups:
/// - `full_name`
/// - `name`
/// - `subcommand_or_group`
/// - `subcommand`
/// - `id`
pub static SLASH_COMMAND_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"<\/(?P<full_name>(?P<name>[-_\p{L}\p{N}\p{sc=Deva}\p{sc=Thai}]{1,32})(?: (?P<subcommand_or_group>[-_\p{L}\p{N}\p{sc=Deva}\p{sc=Thai}]{1,32}))?(?: (?P<subcommand>[-_\p{L}\p{N}\p{sc=Deva}\p{sc=Thai}]{1,32}))?):(?P<id>\d{17,20})>"
    )
    .unwrap()
});

/// Regex for matching a custom emoji (static or animated).
///
/// Available groups:
/// - `animated`
/// - `name`
/// - `id`
pub static EMOJI_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<(?P<animated>a)?:(?P<name>\w{2,32}):(?P<id>\d{17,20})>").unwrap()
});

/// Regex for matching an animated custom emoji.
///
/// Available groups:
/// - `animated`
/// - `name`
/// - `id`
pub static ANIMATED_EMOJI_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<(?P<animated>a):(?P<name>\w{2,32}):(?P<id>\d{17,20})>").unwrap()
});

/// Regex for matching a static custom emoji.
///
/// Available groups:
/// - `name`
/// - `id`
pub static STATIC_EMOJI_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<:(?P<name>\w{2,32}):(?P<id>\d{17,20})>").unwrap());

/// Regex for matching a Discord timestamp.
/// Use [`STYLED_TIMESTAMP_PATTERN`] if the style is required.
///
/// Available groups:
/// - `timestamp`
/// - `style` (optional)
pub static TIMESTAMP_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<t:(?P<timestamp>-?\d{1,19})(:(?P<style>[tTdDfFR]))?>").unwrap());

/// Regex for matching a default-style Discord timestamp.
///
/// Available groups:
/// - `timestamp`
pub static DEFAULT_STYLED_TIMESTAMP_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<t:(?P<timestamp>-?\d{1,19})>").unwrap());

/// Regex for matching a styled Discord timestamp.
///
/// Available groups:
/// - `timestamp`
/// - `style`
pub static STYLED_TIMESTAMP_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<t:(?P<timestamp>-?\d{1,19}):(?P<style>[tTdDfFR])>").unwrap());

/// Regex for matching a guild navigation mention.
///
/// Available groups:
/// - `type`
pub static GUILD_NAVIGATION_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<id:(?P<type>customize|browse|guide|linked-roles)>").unwrap());

/// Regex for matching a linked role mention.
///
/// Available groups:
/// - `id`
pub static LINKED_ROLE_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<id:linked-roles:(?P<id>\d{17,20})>").unwrap());

#[cfg(test)]
mod tests {
    use super::*;

    const ID: &str = "123456789012345678";

    #[test]
    fn user() {
        let input = format!("<@{ID}>");
        let caps = USER_PATTERN.captures(&input).unwrap();

        assert_eq!(&caps["id"], ID);
    }

    #[test]
    fn user_with_nickname() {
        let input = format!("<@!{ID}>");
        let caps = USER_WITH_NICKNAME_PATTERN.captures(&input).unwrap();

        assert_eq!(&caps["id"], ID);
    }

    #[test]
    fn user_with_optional_nickname() {
        let input = format!("<@{ID}>");
        let caps = USER_WITH_OPTIONAL_NICKNAME_PATTERN
            .captures(&input)
            .unwrap();
        assert_eq!(&caps["id"], ID);

        let input = format!("<@!{ID}>");
        let caps = USER_WITH_OPTIONAL_NICKNAME_PATTERN
            .captures(&input)
            .unwrap();
        assert_eq!(&caps["id"], ID);
    }

    #[test]
    fn channel() {
        let input = format!("<#{ID}>");
        let caps = CHANNEL_PATTERN.captures(&input).unwrap();

        assert_eq!(&caps["id"], ID);
    }

    #[test]
    fn role() {
        let input = format!("<@&{ID}>");
        let caps = ROLE_PATTERN.captures(&input).unwrap();

        assert_eq!(&caps["id"], ID);
    }

    #[test]
    fn slash_command() {
        let input = format!("</admin user ban:{ID}>");
        let caps = SLASH_COMMAND_PATTERN.captures(&input).unwrap();

        assert_eq!(&caps["full_name"], "admin user ban");
        assert_eq!(&caps["name"], "admin");
        assert_eq!(&caps["subcommand_or_group"], "user");
        assert_eq!(&caps["subcommand"], "ban");
        assert_eq!(&caps["id"], ID);
    }

    #[test]
    fn emoji() {
        let input = format!("<a:blob:{ID}>");
        let caps = EMOJI_PATTERN.captures(&input).unwrap();

        assert_eq!(&caps["animated"], "a");
        assert_eq!(&caps["name"], "blob");
        assert_eq!(&caps["id"], ID);
    }

    #[test]
    fn animated_emoji() {
        let input = format!("<a:blob:{ID}>");
        let caps = ANIMATED_EMOJI_PATTERN.captures(&input).unwrap();

        assert_eq!(&caps["animated"], "a");
        assert_eq!(&caps["name"], "blob");
        assert_eq!(&caps["id"], ID);
    }

    #[test]
    fn static_emoji() {
        let input = format!("<:blob:{ID}>");
        let caps = STATIC_EMOJI_PATTERN.captures(&input).unwrap();

        assert_eq!(&caps["name"], "blob");
        assert_eq!(&caps["id"], ID);
    }

    #[test]
    fn timestamp() {
        let input = "<t:1700000000:F>";
        let caps = TIMESTAMP_PATTERN.captures(input).unwrap();

        assert_eq!(&caps["timestamp"], "1700000000");
        assert_eq!(&caps["style"], "F");
    }

    #[test]
    fn default_timestamp() {
        let input = "<t:1700000000>";
        let caps = DEFAULT_STYLED_TIMESTAMP_PATTERN.captures(input).unwrap();

        assert_eq!(&caps["timestamp"], "1700000000");
    }

    #[test]
    fn styled_timestamp() {
        let input = "<t:1700000000:R>";
        let caps = STYLED_TIMESTAMP_PATTERN.captures(input).unwrap();

        assert_eq!(&caps["timestamp"], "1700000000");
        assert_eq!(&caps["style"], "R");
    }

    #[test]
    fn guild_navigation() {
        let caps = GUILD_NAVIGATION_PATTERN
            .captures("<id:linked-roles>")
            .unwrap();

        assert_eq!(&caps["type"], "linked-roles");
    }

    #[test]
    fn linked_role() {
        let input = format!("<id:linked-roles:{ID}>");
        let caps = LINKED_ROLE_PATTERN.captures(&input).unwrap();

        assert_eq!(&caps["id"], ID);
    }

    #[test]
    fn invalid_inputs() {
        assert!(USER_PATTERN.captures("<@123>").is_none());
        assert!(CHANNEL_PATTERN.captures("<#abc>").is_none());
        assert!(ROLE_PATTERN.captures("<@&foo>").is_none());
        assert!(STATIC_EMOJI_PATTERN.captures("<:a:123>").is_none());
        assert!(
            ANIMATED_EMOJI_PATTERN
                .captures("<:blob:123456789012345678>")
                .is_none()
        );
    }
}
