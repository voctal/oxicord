use oxicord_snowflake::{AchievementId, ApplicationId, TeamId};

use crate::CdnUrlOptions;

use super::Cdn;

impl Cdn {
    /// Generates an app icon URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn app_icon(
        &self,
        application_id: ApplicationId,
        icon_hash: &str,
        options: CdnUrlOptions,
    ) -> String {
        self.build_cdn_url(&format!("/app-icons/{application_id}/{icon_hash}"), options)
    }

    /// Generates an app cover URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn app_cover(
        &self,
        application_id: ApplicationId,
        cover_hash: &str,
        options: CdnUrlOptions,
    ) -> String {
        self.build_cdn_url(
            &format!("/app-icons/{application_id}/{cover_hash}"),
            options,
        )
    }

    /// Generates an app asset URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn app_asset(
        &self,
        application_id: ApplicationId,
        asset_hash: &str,
        options: CdnUrlOptions,
    ) -> String {
        self.build_cdn_url(
            &format!("/app-assets/{application_id}/{asset_hash}"),
            options,
        )
    }

    /// Generates an app achievement icon URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn achievement_icon(
        &self,
        application_id: ApplicationId,
        achievement_id: AchievementId,
        icon_hash: &str,
        options: CdnUrlOptions,
    ) -> String {
        self.build_cdn_url(
            &format!(
                "/app-assets/{application_id}/achievements/{achievement_id}/icons/{icon_hash}"
            ),
            options,
        )
    }

    /// Generates a team icon URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn team_icon(&self, team_id: TeamId, team_icon: &str, options: CdnUrlOptions) -> String {
        self.build_cdn_url(&format!("/team-icons/{team_id}/{team_icon}"), options)
    }
}
