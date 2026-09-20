use oxicord_snowflake::UserId;

use crate::{CdnUrlOptions, ImageUrlOptions};

use super::Cdn;

impl Cdn {
    /// Generates a user avatar URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn user_avatar(
        &self,
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
            &format!("/avatars/{user_id}/{avatar_hash}"),
            avatar_hash,
            dynamic_options,
        )
    }

    /// Generates a default avatar URL.
    ///
    /// Use [`crate::calculate_user_default_avatar_index`] to get the index from the user ID.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn default_user_avatar(&self, index: u32, options: CdnUrlOptions) -> String {
        self.build_cdn_url(&format!("/embed/avatars/{index}"), options)
    }

    /// Generates a user avatar decoration preset URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn avatar_decoration(&self, asset: &str, options: CdnUrlOptions) -> String {
        self.build_cdn_url(&format!("/avatar-decoration-presets/{asset}"), options)
    }

    /// Generates a banner URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn user_banner(
        &self,
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
            &format!("/banners/{user_id}/{banner_hash}"),
            banner_hash,
            dynamic_options,
        )
    }
}
