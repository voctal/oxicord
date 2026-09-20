use crate::client::Rest;
use crate::events::EVENT_CHANNEL_CAPACITY;
use crate::{API_VERSION, BASE_URL};

use bytes::Bytes;
use http_body_util::Full;
use hyper::header::HeaderName;
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use oxicord_cdn::{CDN_URL, MEDIA_PROXY_URL};
use std::time::Duration;
use tokio::sync::broadcast;

pub(crate) type HyperClient = Client<HttpsConnector<HttpConnector>, Full<Bytes>>;

/// A builder for [`Rest`].
pub struct RestBuilder {
    auth: Option<String>,
    auth_prefix: String,
    user_agent_appendix: Option<String>,
    global_rate_limit: u32,
    max_retries: u32,
    retry_backoff_ms: u64,
    timeout: Duration,
    api_base: String,
    cdn_base: String,
    media_proxy_base: String,
    api_version: u8,
    offset_ms: u64,
    hash_sweep_interval: Duration,
    hash_lifetime: Duration,
    handler_sweep_interval: Duration,
    invalid_request_warning_interval: Duration,
    extra_headers: Vec<(HeaderName, String)>,
}

impl Default for RestBuilder {
    fn default() -> Self {
        Self {
            auth: None,
            auth_prefix: "Bot".to_string(),
            user_agent_appendix: None,
            global_rate_limit: 50,
            max_retries: 3,
            retry_backoff_ms: 200,
            timeout: Duration::from_millis(15_000),
            api_base: BASE_URL.to_string(),
            cdn_base: CDN_URL.to_string(),
            media_proxy_base: MEDIA_PROXY_URL.to_string(),
            api_version: API_VERSION,
            offset_ms: 50,
            hash_sweep_interval: Duration::from_secs(4 * 60 * 60),
            hash_lifetime: Duration::from_secs(24 * 60 * 60),
            handler_sweep_interval: Duration::from_secs(60 * 60),
            invalid_request_warning_interval: Duration::ZERO,
            extra_headers: Vec::new(),
        }
    }
}

impl RestBuilder {
    /// Sets the bot token, used in the `Authorization` header.
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.auth = Some(token.into());
        self
    }

    /// Uses a raw `Authorization` header value, e.g. for `"Bearer <oauth token>"`.
    pub fn authorization(mut self, value: impl Into<String>) -> Self {
        self.auth = Some(value.into());
        self.auth_prefix = String::new();
        self
    }

    /// Prefix used with `token()`, e.g. `"Bot"` (default) or `"Bearer"` for OAuth2 user tokens.
    pub fn auth_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.auth_prefix = prefix.into();
        self
    }

    pub fn user_agent_appendix(mut self, appendix: impl Into<String>) -> Self {
        self.user_agent_appendix = Some(appendix.into());
        self
    }

    pub fn global_rate_limit(mut self, per_second: u32) -> Self {
        self.global_rate_limit = per_second;
        self
    }

    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    pub fn retry_backoff(mut self, ms: u64) -> Self {
        self.retry_backoff_ms = ms;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn api(mut self, base: impl Into<String>) -> Self {
        self.api_base = base.into();
        self
    }

    pub fn cdn(mut self, base: impl Into<String>) -> Self {
        self.cdn_base = base.into();
        self
    }

    pub fn media_proxy(mut self, base: impl Into<String>) -> Self {
        self.media_proxy_base = base.into();
        self
    }

    pub fn api_version(mut self, version: u8) -> Self {
        self.api_version = version;
        self
    }

    pub fn offset(mut self, ms: u64) -> Self {
        self.offset_ms = ms;
        self
    }

    pub fn hash_sweep_interval(mut self, interval: Duration) -> Self {
        self.hash_sweep_interval = interval;
        self
    }

    pub fn hash_lifetime(mut self, lifetime: Duration) -> Self {
        self.hash_lifetime = lifetime;
        self
    }

    pub fn handler_sweep_interval(mut self, interval: Duration) -> Self {
        self.handler_sweep_interval = interval;
        self
    }

    pub fn invalid_request_warning_interval(mut self, interval: Duration) -> Self {
        self.invalid_request_warning_interval = interval;
        self
    }

    pub fn header(mut self, name: HeaderName, value: impl Into<String>) -> Self {
        self.extra_headers.push((name, value.into()));
        self
    }

    /// Builds the client.
    pub fn build(self) -> Rest {
        let connector = HttpsConnectorBuilder::new()
            .with_native_roots()
            .expect("failed to load native TLS root certificates")
            .https_only()
            .enable_http1()
            .build();
        let client = Client::builder(TokioExecutor::new()).build(connector);

        let mut user_agent = format!("DiscordBot (oxicord, v{})", env!("CARGO_PKG_VERSION"));
        if let Some(appendix) = &self.user_agent_appendix {
            user_agent.push(' ');
            user_agent.push_str(appendix);
        }

        let auth = self.auth.map(|token| {
            if self.auth_prefix.is_empty() {
                token
            } else {
                format!("{} {token}", self.auth_prefix)
            }
        });

        let (events_tx, _) = broadcast::channel(EVENT_CHANNEL_CAPACITY);

        Rest {
            client,
            auth,
            user_agent,
            max_retries: self.max_retries,
            timeout: self.timeout,
            api_base: format!(
                "{}/v{}",
                self.api_base.trim_end_matches('/'),
                self.api_version
            ),
            api_version: self.api_version,
            cdn_base: self.cdn_base,
            media_proxy_base: self.media_proxy_base,
            extra_headers: self.extra_headers,
            events_tx,
        }
    }
}
