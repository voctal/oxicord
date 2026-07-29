//! Guild navigation helpers (`<id:TYPE>`).

/// Returns the guild customization tab `<id:customize>` (Channels & Roles tab).
///
/// See the [Discord Message Formatting](https://docs.discord.com/developers/reference#message-formatting) docs.
pub fn guild_customize_link() -> &'static str {
    "<id:customize>"
}

/// Returns the guild browse channels tab `<id:browse>`.
///
/// See the [Discord Message Formatting](https://docs.discord.com/developers/reference#message-formatting) docs.
pub fn guild_browse_link() -> &'static str {
    "<id:browse>"
}

/// Returns the guild guide tab `<id:guide>`.
///
/// See the [Discord Message Formatting](https://docs.discord.com/developers/reference#message-formatting) docs.
pub fn guild_guide_link() -> &'static str {
    "<id:guide>"
}

/// Returns the guild linked-roles tab `<id:linked-roles>`.
///
/// See the [Discord Message Formatting](https://docs.discord.com/developers/reference#message-formatting) docs.
pub fn giuld_linked_roles_link() -> &'static str {
    "<id:linked-roles>"
}

/// Returns the link of a specific linked role, opening the connection modal on click.
/// `<id:linked-roles:role_id>`
///
/// See the [Discord Message Formatting](https://docs.discord.com/developers/reference#message-formatting) docs.
pub fn guild_linked_role_link(role_id: u64) -> String {
    format!("<id:linked-roles:{role_id}>")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_role_formatters() {
        assert_eq!(guild_linked_role_link(123), "<id:linked-roles:123>");
    }
}
