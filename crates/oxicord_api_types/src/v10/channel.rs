use oxicord_macros::{discord_enum, discord_type};

use crate::Snowflake;

/// <https://discord.com/developers/docs/resources/channel#channel-object-channel-types>
#[discord_enum(u8)]
pub enum ChannelType {
    /// A text channel within a guild
    GuildText = 0,
    /// A direct message between users
    Dm = 1,
    /// A voice channel within a guild
    GuildVoice = 2,
    /// A direct message between multiple users
    GroupDm = 3,
    /// An organizational category that contains up to 50 channels
    GuildCategory = 4,
    /// A channel that users can follow and crosspost into their own guild
    GuildAnnouncement = 5,
    /// A temporary sub-channel within a Guild Announcement channel
    AnnouncementThread = 10,
    /// A temporary sub-channel within a Guild Text or Guild Forum channel
    PublicThread = 11,
    /// A temporary sub-channel within a Guild Text channel that is only viewable by those invited and those with the Manage Threads permission
    PrivateThread = 12,
    /// A voice channel for hosting events with an audience
    GuildStageVoice = 13,
    /// The channel in a Student Hub containing the listed servers
    GuildDirectory = 14,
    /// A channel that can only contain threads
    GuildForum = 15,
    /// A channel like forum channels but contains media for server subscriptions
    GuildMedia = 16,
}

#[discord_type]
pub struct ApiPartialChannel {
    /// The id of the channel
    pub id: Snowflake,
    /// The type of the channel
    #[serde(rename = "type")]
    pub channel_type: ChannelType,
    /// The name of the channel
    #[serde(default)]
    pub name: Option<String>,
}
