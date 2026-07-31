//! Gateway types.

use super::user::ApiUser;
use crate::Snowflake;

use oxicord_macros::discord_type;
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
    pub id: Snowflake,
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
