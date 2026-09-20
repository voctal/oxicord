use oxicord_snowflake::{ChannelId, GuildId, RoleId, ScheduledEventId};

use crate::{CdnUrlOptions, ImageUrlOptions};

use super::Cdn;

impl Cdn {
    /// Generates a channel icon URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn channel_icon(
        &self,
        channel_id: ChannelId,
        icon_hash: &str,
        options: CdnUrlOptions,
    ) -> String {
        self.build_cdn_url(&format!("/channel-icons/{channel_id}/{icon_hash}"), options)
    }

    /// Generates a role icon URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn role_icon(
        &self,
        role_id: RoleId,
        role_icon_hash: &str,
        options: CdnUrlOptions,
    ) -> String {
        self.build_cdn_url(&format!("/role-icons/{role_id}/{role_icon_hash}"), options)
    }

    /// Generates a guild icon URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn guild_icon(&self, guild_id: GuildId, icon_hash: &str, options: CdnUrlOptions) -> String {
        let dynamic_options = ImageUrlOptions {
            extension: options.extension,
            size: options.size,
            force_static: false,
        };

        self.build_dynamic_url(
            &format!("/icons/{guild_id}/{icon_hash}"),
            icon_hash,
            dynamic_options,
        )
    }

    /// Generates a guild banner URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn guild_banner(
        &self,
        guild_id: GuildId,
        banner_hash: &str,
        options: CdnUrlOptions,
    ) -> String {
        let dynamic_options = ImageUrlOptions {
            extension: options.extension,
            size: options.size,
            force_static: false,
        };

        self.build_dynamic_url(
            &format!("/banners/{guild_id}/{banner_hash}"),
            banner_hash,
            dynamic_options,
        )
    }

    /// Generates a guild splash URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn guild_splash(
        &self,
        guild_id: GuildId,
        splash_hash: &str,
        options: CdnUrlOptions,
    ) -> String {
        self.build_cdn_url(&format!("/splashes/{guild_id}/{splash_hash}"), options)
    }

    /// Generates a guild discovery splash URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn guild_discovery_splash(
        &self,
        guild_id: GuildId,
        splash_hash: &str,
        options: CdnUrlOptions,
    ) -> String {
        self.build_cdn_url(
            &format!("/discovery-splashes/{guild_id}/{splash_hash}"),
            options,
        )
    }

    /// Generates a scheduled event cover URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn guild_scheduled_event_cover(
        &self,
        scheduled_event_id: ScheduledEventId,
        cover_hash: &str,
        options: CdnUrlOptions,
    ) -> String {
        self.build_cdn_url(
            &format!("/guild-events/{scheduled_event_id}/{cover_hash}"),
            options,
        )
    }

    /// Generates a guild tag badge URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn guild_tag_badge(
        &self,
        guild_id: GuildId,
        badge_hash: &str,
        options: CdnUrlOptions,
    ) -> String {
        self.build_cdn_url(
            &format!("/guild-tag-badges/{guild_id}/{badge_hash}"),
            options,
        )
    }
}
