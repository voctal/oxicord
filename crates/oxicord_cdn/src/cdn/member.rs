use oxicord_snowflake::{GuildId, UserId};

use crate::{CdnUrlOptions, ImageUrlOptions};

use super::Cdn;

impl Cdn {
    /// Generates a guild member avatar URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn guild_member_avatar(
        &self,
        guild_id: GuildId,
        user_id: UserId,
        avatar_hash: &str,
        options: CdnUrlOptions,
    ) -> String {
        let dynamic_options = ImageUrlOptions {
            extension: options.extension,
            size: options.size,
            force_static: false,
        };

        self.build_dynamic_url(
            &format!("/guilds/{guild_id}/users/{user_id}/avatars/{avatar_hash}"),
            avatar_hash,
            dynamic_options,
        )
    }

    /// Generates a guild member banner URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn guild_member_banner(
        &self,
        guild_id: GuildId,
        user_id: UserId,
        banner_hash: &str,
        options: CdnUrlOptions,
    ) -> String {
        let dynamic_options = ImageUrlOptions {
            extension: options.extension,
            size: options.size,
            force_static: false,
        };

        self.build_dynamic_url(
            &format!("/guilds/{guild_id}/users/{user_id}/banners/{banner_hash}"),
            banner_hash,
            dynamic_options,
        )
    }
}
