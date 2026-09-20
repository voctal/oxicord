use std::time::Duration;

use hyper::Method;

pub const BURST_MAJOR_ID_KEY: &str = "burst";

/// Identifies a request for rate-limiting purposes.
#[derive(Debug, Clone)]
pub struct Endpoint {
    pub method: Method,
    /// e.g. `channels/:id/messages/:id`
    pub route: String,
    /// The major parameter: a channel/guild/webhook id, or `"global"` for routes with
    /// none (e.g. `/gateway`). Two different ids are always different buckets even if
    /// `route` and the eventual Discord-reported hash are identical.
    pub major_parameter: String,
}

#[derive(Debug, Clone, Default)]
pub struct ResponseInfo {
    /// Present whenever Discord sent `X-RateLimit-*` headers (normally every response,
    /// success or not).
    pub bucket: Option<BucketHeaders>,
    /// Present only on a 429.
    pub too_many_requests: Option<TooManyRequests>,
}

#[derive(Debug, Clone)]
pub struct BucketHeaders {
    pub hash: String,
    pub limit: i64,
    pub remaining: i64,
    pub reset_after: Duration,
    pub scope: RateLimitScope,
}

#[derive(Debug, Clone)]
pub struct TooManyRequests {
    pub retry_after: Duration,
    pub scope: RateLimitScope,
    pub global: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RateLimitScope {
    #[default]
    User,
    Global,
    Shared,
}

impl RateLimitScope {
    pub fn parse(s: &str) -> Self {
        match s {
            "global" => Self::Global,
            "shared" => Self::Shared,
            _ => Self::User,
        }
    }
}

/// Ratelimit data.
#[derive(Debug, Clone)]
pub struct RateLimitData {
    /// Whether this is the global limit or a per-route bucket limit.
    pub global: bool,
    /// The bucket hash for this request, if known yet (empty before the first response).
    pub hash: String,
    /// Requests allowed per window for this bucket, if known (-1 until the first response).
    pub limit: i64,
    /// The major parameter of the route (either an id, or `"global"`).
    pub major_parameter: String,
    /// The HTTP method that was used.
    pub method: Method,
    /// Milliseconds until this specific request can be retried.
    pub retry_after: u64,
    /// The bucket route key, e.g. `channels/:id/messages/:id`.
    pub route: String,
    /// `X-RateLimit-Scope` header when present. See [`RateLimitScope`].
    pub scope: RateLimitScope,
    /// ID of the sublimit rule that applied, if any (see [`super::SublimitRule`]).
    pub sublimit_id: Option<&'static str>,
    /// Milliseconds until a sublimit resets, if this request was throttled by one.
    pub sublimit_timeout: u64,
    /// Milliseconds until the bucket's window resets.
    pub time_to_reset: u64,
}
