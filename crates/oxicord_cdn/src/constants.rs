/// Base URL for the Discord CDN.
pub const CDN_URL: &str = "https://cdn.discordapp.com";

/// Discord media proxy URL.
pub const MEDIA_PROXY_URL: &str = "https://media.discordapp.net";

/// Some MIME types have to be remapped, see <https://github.com/discord/discord-api-docs/issues/5390>.
pub const OVERWRITTEN_MIME_TYPES: &[(&str, &str)] = &[("image/apng", "image/png")];

/// Allowed image extensions for the Discord CDN images.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageExtension {
    #[default]
    Webp,
    Png,
    Jpeg,
    Gif,
}

impl ImageExtension {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Webp => "webp",
            Self::Png => "png",
            Self::Jpeg => "jpeg",
            Self::Gif => "gif",
        }
    }
}

/// Allowed image sizes for the Discord CDN.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ImageSize {
    S16 = 16,
    S32 = 32,
    S64 = 64,
    S128 = 128,
    S256 = 256,
    S512 = 512,
    S1024 = 1024,
    S2048 = 2048,
    S4096 = 4096,
}

impl ImageSize {
    pub const fn as_u16(self) -> u16 {
        self as u16
    }
}

/// Options to build a CDN url.
#[derive(Debug, Clone, Copy, Default)]
pub struct CdnUrlOptions {
    pub extension: Option<ImageExtension>,
    pub size: Option<ImageSize>,
}

/// Options to build an image URL.
#[derive(Debug, Clone, Copy)]
pub struct ImageUrlOptions {
    pub extension: Option<ImageExtension>,
    pub size: Option<ImageSize>,
    pub force_static: bool,
}

impl Default for ImageUrlOptions {
    fn default() -> Self {
        Self {
            extension: Some(ImageExtension::Webp),
            size: None,
            force_static: false,
        }
    }
}

/// Allowed sticker extensions for the Discord CDN.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StickerExtension {
    Png,
    Gif,
    Lottie,
}

impl StickerExtension {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Gif => "gif",
            Self::Lottie => "json",
        }
    }
}
