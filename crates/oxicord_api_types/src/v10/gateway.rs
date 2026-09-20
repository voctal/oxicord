//! Gateway types.

use super::user::ApiUser;

use oxicord_macros::{discord_bitflags, discord_enum, discord_type};
use oxicord_snowflake::ApplicationId;
use serde::{Deserialize, Serialize};

/// The `d` payload of an `Identify` (opcode 2) gateway command.
///
/// <https://discord.com/developers/docs/events/gateway-events#identify>
#[discord_type]
pub struct GatewayIdentify {
    /// Authentication token.
    pub token: String,
    /// Connection properties.
    pub properties: GatewayIdentifyConnectionProperties,
    /// Whether this connection supports compression of packets.
    #[serde(default)]
    pub compress: Option<bool>,
    #[serde(default)]
    pub large_threshold: Option<u8>,
    /// Used for Guild Sharding.
    #[serde(default)]
    pub shard: Option<(u32, u32)>,
    /// Gateway Intents you wish to receive.
    pub intents: u64,
}

#[discord_type]
pub struct GatewayIdentifyConnectionProperties {
    /// Your operating system.
    pub os: String,
    /// Your library name.
    pub browser: String,
    /// Your library name.
    pub device: String,
}

/// The `d` payload of the `READY` dispatch event.
///
/// <https://discord.com/developers/docs/events/gateway-events#ready>
#[discord_type]
pub struct GatewayReadyDispatchData {
    /// API version.
    pub v: u8,
    /// Information about the user including email.
    pub user: ApiUser,
    /// Used for resuming connections.
    pub session_id: String,
    /// Gateway URL for resuming connections.
    pub resume_gateway_url: String,
    /// Shard information associated with this session, if sent when identifying.
    pub shard: Option<(u32, u32)>,
    /// Partial application.
    pub application: GatewayReadyApplication,
}

#[discord_type]
pub struct GatewayReadyApplication {
    pub id: ApplicationId,
    #[serde(default)]
    pub flags: Option<u64>,
}

/// https://docs.discord.com/developers/events/gateway-events#payload-structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GatewayPayload {
    /// Gateway opcode, which indicates the payload type.
    pub op: u8,
    /// Event data.
    #[serde(default)]
    pub d: Option<serde_json::Value>,
    /// Sequence number of event used for resuming sessions and heartbeating.
    #[serde(default)]
    pub s: Option<u64>,
    /// Event name.
    #[serde(default)]
    pub t: Option<String>,
}

/// <https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-opcodes>
#[discord_enum(u8)]
pub enum GatewayOpcode {
    /// An event was dispatched.
    ///
    /// Client Action: Receive
    Dispatch = 0,
    /// Fired periodically by the client to keep the connection alive.
    ///
    /// Client Action: Send/Receive
    Heartbeat = 1,
    ///	Starts a new session during the initial handshake.
    ///
    /// Client Action: Send
    Identify = 2,
    /// Update the client's presence.
    ///
    /// Client Action: Send
    PresenceUpdate = 3,
    /// Used to join/leave or move between voice channels.
    ///
    /// Client Action: Send
    VoiceStateUpdate = 4,
    /// Resume a previous session that was disconnected.
    ///
    /// Client Action: Send
    Resume = 6,
    /// You should attempt to reconnect and resume immediately.
    ///
    /// Client Action: Receive
    Reconnect = 7,
    /// Request information about offline guild members in a large guild.
    ///
    /// Client Action: Send
    RequestGuildMembers = 8,
    /// The session has been invalidated. You should reconnect and identify/resume accordingly.
    ///
    /// Client Action: Receive
    InvalidSession = 9,
    /// Sent immediately after connecting, contains the `heartbeat_interval` to use.
    ///
    /// Client Action: Receive
    Hello = 10,
    /// Sent in response to receiving a heartbeat to acknowledge that it has been received.
    ///
    /// Client Action: Receive
    HeartbeatAck = 11,
    /// Request information about soundboard sounds in a set of guilds.
    ///
    /// Client Action: Send
    RequestSoundboardSounds = 31,
    /// Request ephemeral channel data for channels in a guild.
    ///
    /// Client Action: Send
    RequestChannelInfo = 43,
}

/// <https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-close-event-codes>
#[discord_enum(u16)]
pub enum GatewayCloseCodes {
    ///	We're not sure what went wrong. Try reconnecting?
    ///
    /// Should reconnect: true
    UnknownError = 4_000,
    /// You sent an invalid Gateway opcode or an invalid payload for an opcode. Don't do that!
    ///
    /// See https://docs.discord.com/developers/topics/opcodes-and-status-codes#gateway-gateway-opcodes
    ///
    /// Should reconnect: true
    UnknownOpcode = 4_001,
    /// You sent an invalid payload to Discord. Don't do that!
    ///
    /// See https://docs.discord.com/developers/events/gateway#sending-events
    ///
    /// Should reconnect: true
    DecodeError = 4_002,
    /// You sent us a payload prior to identifying, or this session has been invalidated.
    ///
    /// See https://docs.discord.com/developers/events/gateway#identifying
    ///
    /// Should reconnect: true
    NotAuthenticated = 4_003,
    /// The account token sent with your identify payload is incorrect.
    ///
    /// See https://docs.discord.com/developers/events/gateway-events#identify
    ///
    /// Should reconnect: true
    AuthenticationFailed = 4_004,
    /// You sent more than one identify payload. Don't do that!
    ///
    /// Should reconnect: true
    AlreadyAuthenticated = 4_005,
    /// The sequence sent when resuming the session was invalid. Reconnect and start a new session.
    ///
    /// See https://docs.discord.com/developers/events/gateway-events#resume
    ///
    /// Should reconnect: true
    InvalidSeq = 4_007,
    /// Woah nelly! You're sending payloads to us too quickly. Slow it down! You will be disconnected on receiving this.
    ///
    /// Should reconnect: true
    RateLimited = 4_008,
    /// Your session timed out. Reconnect and start a new one.
    ///
    /// Should reconnect: true
    SessionTimedOut = 4_009,
    /// You sent us an invalid shard when identifying.
    ///
    /// See https://docs.discord.com/developers/events/gateway#sharding
    ///
    /// Should reconnect: false
    InvalidShard = 4_010,
    /// The session would have handled too many guilds - you are required to shard your connection in order to connect.
    ///
    /// See https://docs.discord.com/developers/events/gateway#sharding
    ///
    /// Should reconnect: false
    ShardingRequired = 4_011,
    /// You sent an invalid version for the gateway.
    ///
    /// Should reconnect: false
    InvalidAPIVersion = 4_012,
    /// You sent an invalid intent for a Gateway Intent. You may have incorrectly calculated the bitwise value.
    ///
    /// See https://docs.discord.com/developers/events/gateway#gateway-intents
    ///
    /// Should reconnect: false
    InvalidIntents = 4_013,
    /// You sent a disallowed intent for a Gateway Intent.
    /// You may have tried to specify an intent that you have not enabled or are not approved for.
    ///
    /// See https://docs.discord.com/developers/events/gateway#gateway-intents
    ///
    /// See https://docs.discord.com/developers/events/gateway#privileged-intents
    ///
    /// Should reconnect: false
    DisallowedIntents = 4_014,
}

discord_bitflags! {
    /// Gateway capabilities.
    ///
    /// <https://docs.discord.com/developers/events/gateway-events#identify-gateway-capabilities>
    pub struct GatewayCapabilities: u64 {
        /// Opts the client into receiving obfuscated channel metadata over the Gateway for channels it can't view.
        const CHANNEL_OBFUSCATION = 1 << 15;
    }
}

discord_bitflags! {
    /// Gateway intents.
    ///
    /// <https://discord.com/developers/docs/topics/gateway#list-of-intents>
    pub struct GatewayIntents: u64 {
        const Guilds = 1 << 0;
        const GuildMembers = 1 << 1;
        const GuildModeration = 1 << 2;
        const GuildExpressions = 1 << 3;
        const GuildIntegrations = 1 << 4;
        const GuildWebhooks = 1 << 5;
        const GuildInvites = 1 << 6;
        const GuildVoiceStates = 1 << 7;
        const GuildPresences = 1 << 8;
        const GuildMessages = 1 << 9;
        const GuildMessageReactions = 1 << 10;
        const GuildMessageTyping = 1 << 11;
        const DirectMessages = 1 << 12;
        const DirectMessageReactions = 1 << 13;
        const DirectMessageTyping = 1 << 14;
        const MessageContent = 1 << 15;
        const GuildScheduledEvents = 1 << 16;
        const AutoModerationConfiguration = 1 << 20;
        const AutoModerationExecution = 1 << 21;
        const GuildMessagePolls = 1 << 24;
        const DirectMessagePolls = 1 << 25;
    }
}
