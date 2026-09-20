use url::Url;

use crate::{CDN_URL, CdnUrlOptions, ImageExtension, ImageUrlOptions, MEDIA_PROXY_URL};

mod app;
mod expression;
mod guild;
mod member;
mod user;

/// CDN link builder.
///
/// See the endpoints documentation at <https://docs.discord.com/developers/reference#image-formatting-cdn-endpoints>.
#[derive(Debug)]
pub struct Cdn {
    cdn_url: String,
    media_proxy_url: String,
}

impl Cdn {
    /// Creates a CDN URL builder.
    pub fn new(cdn: Option<impl Into<String>>, media_proxy: Option<impl Into<String>>) -> Self {
        Self {
            cdn_url: cdn.map(Into::into).unwrap_or_else(|| CDN_URL.to_string()),
            media_proxy_url: media_proxy
                .map(Into::into)
                .unwrap_or_else(|| MEDIA_PROXY_URL.to_string()),
        }
    }

    /// Get the current CDN URL.
    #[inline]
    pub fn cdn_url(&self) -> &str {
        &self.cdn_url
    }

    /// Get the current media proxy URL.
    #[inline]
    pub fn media_proxy_url(&self) -> &str {
        &self.media_proxy_url
    }
}

impl Default for Cdn {
    fn default() -> Self {
        Self::new(Option::<&str>::None, Option::<&str>::None)
    }
}

impl Cdn {
    /// Builds a CDN URL.
    #[inline]
    pub fn build_cdn_url(&self, route: &str, options: CdnUrlOptions) -> String {
        self.build_url_from(&self.media_proxy_url, route, options)
    }

    /// Builds a media URL.
    #[inline]
    pub fn build_media_url(&self, route: &str, options: CdnUrlOptions) -> String {
        self.build_url_from(&self.media_proxy_url, route, options)
    }

    /// Builds a URL from the given base (cdn and medias use different base).
    ///
    /// Use [`Cdn::dynamic_make_url`] for possibly animated assets.
    ///
    /// # Panics
    ///
    /// Panics if the base URL is invalid.
    pub fn build_url_from(&self, base: &str, route: &str, options: CdnUrlOptions) -> String {
        let extension = options.extension.unwrap_or(ImageExtension::Webp);

        let base = base.trim_end_matches('/');
        let route = route.trim_start_matches('/');

        let mut url = Url::parse(&format!("{base}/{route}.{}", extension.as_str()))
            .expect("Discord CDN base URL should always be valid");

        if let Some(size) = options.size {
            url.query_pairs_mut()
                .append_pair("size", &size.as_u16().to_string());
        }

        url.into()
    }

    /// Builds a URL for an asset that may be animated.
    ///
    /// Discord uses hashes starting with `a_` to indicate animated assets.
    ///
    /// # Panics
    ///
    /// Can panics if [`Cdn::make_url_from`] panics.
    pub fn build_dynamic_url(&self, route: &str, hash: &str, options: ImageUrlOptions) -> String {
        let animated = hash.starts_with("a_") && !options.force_static;

        let mut url = self.build_cdn_url(
            route,
            CdnUrlOptions {
                extension: options.extension,
                size: options.size,
            },
        );

        if animated {
            let mut parsed =
                Url::parse(&url).expect("URL produced by make_url_from should always be valid");

            parsed.query_pairs_mut().append_pair("animated", "true");
            url = parsed.into();
        }

        url
    }
}
