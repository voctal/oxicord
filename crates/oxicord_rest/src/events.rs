use bytes::Bytes;
use hyper::{Method, StatusCode};

use crate::RateLimitData;

/// A [`Rest`](crate::Rest) event, received using [`Rest::subscribe`](crate::Rest::subscribe).
#[derive(Debug, Clone)]
pub enum RestEvent {
    /// A request was retried or rejected because of a ratelimit.
    RateLimited(Box<RateLimitData>),
    /// A response.
    Response {
        route: String,
        method: Method,
        status: StatusCode,
        body: Bytes,
    },
    /// A 401/403/429 was seen.
    InvalidRequestWarning { count: u32 },
}

pub(crate) const EVENT_CHANNEL_CAPACITY: usize = 256;
