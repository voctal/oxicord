use oxicord_snowflake::EmojiId;

use crate::{CdnUrlOptions, ImageExtension, StickerExtension};

use super::Cdn;

impl Cdn {
    /// Generates a custom emoji URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn emoji(&self, emoji_id: EmojiId, options: CdnUrlOptions) -> String {
        self.build_cdn_url(&format!("/emojis/{emoji_id}"), options)
    }

    /// Generates a sticker URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn sticker(
        &self,
        sticker_id: &str,
        extension: StickerExtension,
        options: CdnUrlOptions,
    ) -> String {
        let options = CdnUrlOptions {
            extension: Some(match extension {
                StickerExtension::Png | StickerExtension::Lottie => ImageExtension::Png,
                StickerExtension::Gif => ImageExtension::Gif,
            }),
            size: options.size,
        };

        if extension == StickerExtension::Gif {
            self.build_media_url(&format!("/stickers/{sticker_id}"), options)
        } else {
            self.build_cdn_url(&format!("/stickers/{sticker_id}"), options)
        }
    }

    /// Generates a sticker pack banner URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    ///
    /// TODO: What ID 'type' is banner_id?
    pub fn sticker_pack_banner(&self, banner_id: u64, options: CdnUrlOptions) -> String {
        self.build_cdn_url(
            &format!("/app-assets/710982414301790216/store/{banner_id}"),
            options,
        )
    }

    /// Generates a soundboard sound URL.
    ///
    /// <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>
    pub fn soundboard_sound(&self, sound_id: &str) -> String {
        format!("{}/sounds/{}.ogg", self.cdn_url(), sound_id)
    }
}
