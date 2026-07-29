/// Formats a channel link `https://discord.com/channels/{guild_id}/{channel_id}`.
/// Use `guild_id = None` for DM channels (uses `@me`).
pub fn channel_link(guild_id: Option<u64>, channel_id: u64) -> String {
    match guild_id {
        Some(guild_id) => format!("https://discord.com/channels/{guild_id}/{channel_id}"),
        None => format!("https://discord.com/channels/@me/{channel_id}"),
    }
}

/// Formats a message link `https://discord.com/channels/{guild_id}/{channel_id}/{message_id}`.
/// Use `guild_id = None` for DM channels (uses `@me`).
pub fn message_link(guild_id: Option<u64>, channel_id: u64, message_id: u64) -> String {
    match guild_id {
        Some(guild_id) => {
            format!("https://discord.com/channels/{guild_id}/{channel_id}/{message_id}")
        }
        None => format!("https://discord.com/channels/@me/{channel_id}/{message_id}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_formatters() {
        assert_eq!(
            channel_link(Some(123), 456),
            "https://discord.com/channels/123/456"
        );
        assert_eq!(
            channel_link(None, 123),
            "https://discord.com/channels/@me/123"
        );
        assert_eq!(
            message_link(Some(123), 456, 789),
            "https://discord.com/channels/123/456/789"
        );
        assert_eq!(
            message_link(None, 123, 456),
            "https://discord.com/channels/@me/123/456"
        );
    }
}
