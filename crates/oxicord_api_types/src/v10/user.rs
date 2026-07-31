use oxicord_macros::{discord_enum, discord_type};
use serde::{Deserialize, Serialize};

use crate::Snowflake;

/// A Discord user.
///
/// <https://discord.com/developers/docs/resources/user#user-object>
#[discord_type]
pub struct ApiUser {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    #[serde(default)]
    pub bot: Option<bool>,
    #[serde(default)]
    pub system: Option<bool>,
    #[serde(default)]
    pub mfa_enabled: Option<bool>,
    #[serde(default)]
    pub banner: Option<String>,
    #[serde(default)]
    pub accent_color: Option<u32>,
    #[serde(default)]
    pub locale: Option<String>,
    #[serde(default)]
    pub verified: Option<bool>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub flags: Option<UserFlags>,
    #[serde(default)]
    pub premium_type: Option<UserPremiumType>,
    #[serde(default)]
    pub public_flags: Option<UserFlags>,
}

bitflags::bitflags! {
    /// User account flags.
    ///
    /// <https://discord.com/developers/docs/resources/user#user-object-user-flags>
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct UserFlags: u64 {
        /// Discord Employee
        const STAFF = 1 << 0;
        /// Partnered Server Owner
        const PARTNER = 1 << 1;
        /// HypeSquad Events Member
        const HYPESQUAD = 1 << 2;
        const BUG_HUNTER_LEVEL_1 = 1 << 3;
        /// Undocumented
        const MFASMS = 1 << 4;
        /// House Bravery Member
        const HYPESQUAD_ONLINE_HOUSE_1 = 1 << 6;
        /// House Brilliance Member
        const HYPESQUAD_ONLINE_HOUSE_2 = 1 << 7;
        /// House Balance Member
        const HYPESQUAD_ONLINE_HOUSE_3 = 1 << 8;
        /// Early Nitro Supporter
        const PREMIUM_EARLY_SUPPORTER = 1 << 9;
        /// User is a team
        const TEAM_PSEUDO_USER = 1 << 10;
        /// Undocumented
        const HAS_UNREAD_URGENT_MESSAGES = 1 << 13;
        const BUG_HUNTER_LEVEL_2 = 1 << 14;
        const VERIFIED_BOT = 1 << 16;
        const VERIFIED_DEVELOPER = 1 << 17;
        /// Moderator Programs Alumni
        const CERTIFIED_MODERATOR = 1 << 18;
        /// Bot uses only interactions and is shown in the online member list
        const BOT_HTTP_INTERACTIONS = 1 << 19;
        /// Undocumented; User has been identified as spammer
        const SPAMMER = 1 << 20;
        const ACTIVE_DEVELOPER = 1 << 22;
        /// Undocumented; The user account has been quarantined based on recent activity
        const QUARANTINED = 1 << 44;
        /// Undocumented
        const COLLABORATOR = 1 << 50;
        /// Undocumented
        const RESTRICTED_COLLABORATOR = 1 << 51;
    }
}

/// https://discord.com/developers/docs/resources/user#user-object-premium-types
#[discord_enum(u8)]
pub enum UserPremiumType {
    None = 0,
    NitroClassic = 1,
    Nitro = 2,
    NitroBasic = 3,
}
