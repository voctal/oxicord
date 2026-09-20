use super::builder::HyperClient;
pub use super::builder::RestBuilder;

use crate::RateLimitData;
use crate::RateLimitScope;
use crate::TooManyRequests;
use crate::error::RestError;
use crate::events::RestEvent;
use crate::ratelimit::BURST_MAJOR_ID_KEY;
use crate::routing::Request;

use bytes::Bytes;
use http_body_util::BodyExt;
use http_body_util::Full;
use hyper::header::HeaderName;
use hyper::{HeaderMap, Method};
use serde::de::DeserializeOwned;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::time::timeout;
use urlencoding::encode;

/// A Discord REST client.
pub struct Rest {
    pub(crate) client: HyperClient,
    pub(crate) auth: Option<String>,
    pub(crate) user_agent: String,
    #[allow(unused)]
    pub(crate) max_retries: u32,
    pub(crate) timeout: Duration,
    pub(crate) api_base: String,
    pub(crate) api_version: u8,
    pub(crate) cdn_base: String,
    pub(crate) media_proxy_base: String,
    pub(crate) extra_headers: Vec<(HeaderName, String)>,
    pub(crate) events_tx: broadcast::Sender<RestEvent>,
}

impl Rest {
    /// Creates a client authenticated as a bot (`Authorization: Bot <token>`).
    pub fn new(token: impl Into<String>) -> Self {
        RestBuilder::default().token(token).build()
    }

    /// Creates a client with no authorization header (e.g. for webhook-only usage).
    pub fn without_token() -> Self {
        RestBuilder::default().build()
    }

    /// Subscribes to [`RestEvent`]s.
    pub fn subscribe(&self) -> broadcast::Receiver<RestEvent> {
        self.events_tx.subscribe()
    }

    pub fn cdn_base(&self) -> &str {
        &self.cdn_base
    }

    pub fn media_proxy_base(&self) -> &str {
        &self.media_proxy_base
    }

    /// The API version this client was configured with (see
    /// [`RestBuilder::api_version`]), not necessarily [`crate::API_VERSION`].
    pub fn api_version(&self) -> u8 {
        self.api_version
    }

    pub(crate) fn emit(&self, event: RestEvent) {
        let _ = self.events_tx.send(event);
    }

    /// Executes a request and deserializes the JSON response body into `T`.
    pub async fn execute<T: DeserializeOwned>(&self, req: Request) -> Result<T, RestError> {
        let bytes = self.execute_raw(req).await?;
        if bytes.is_empty() {
            return serde_json::from_slice(b"null").map_err(RestError::Deserialize);
        }
        serde_json::from_slice(&bytes).map_err(RestError::Deserialize)
    }

    /// Executes a request, returning the raw response bytes without deserializing.
    pub async fn execute_raw(&self, req: Request) -> Result<Bytes, RestError> {
        if let Some(e) = req.body_error.clone() {
            return Err(RestError::Serialize(e));
        }

        // let mut attempt = 0;
        // loop {
        match self.execute_once(req.clone()).await {
            Ok(bytes) => return Ok(bytes),
            // Err(RestError::RateLimited(_)) if attempt < self.max_retries => {
            //     attempt += 1;
            //     tracing::debug!(method = %req.method, path = %req.path, attempt, "retrying after rate limit");
            //     continue; // execute_once already slept out the reset window
            // }
            // Err(RestError::ApiError { status, .. })
            //     if status.is_server_error() && attempt < self.max_retries =>
            // {
            //     attempt += 1;
            //     let backoff = self.ratelimiter.retry_backoff(attempt);
            //     tracing::debug!(method = %req.method, path = %req.path, %status, attempt, ?backoff, "retrying after server error");
            //     sleep(backoff).await;
            //     continue;
            // }
            Err(e) => return Err(e),
        }
        // }
    }

    async fn execute_once(&self, req: Request) -> Result<Bytes, RestError> {
        let uri = format!("{}{}", self.api_base, req.full_path());

        // let endpoint = Endpoint {
        //     method: req.method.clone(),
        //     route: req.route.clone(),
        //     major_parameter: req.major_param.clone(),
        // };

        let mut builder = hyper::Request::builder()
            .method(req.method.clone())
            .uri(uri.as_str())
            .header("User-Agent", &self.user_agent);

        for (name, value) in &self.extra_headers {
            builder = builder.header(name, value.as_str());
        }

        if let Some(auth) = &self.auth {
            builder = builder.header("Authorization", auth);
        }
        if let Some(reason) = &req.audit_log_reason {
            builder = builder.header("X-Audit-Log-Reason", encode(reason).into_owned());
        }

        let body_bytes = if let Some(body) = &req.body {
            builder = builder.header("Content-Type", "application/json");
            body.clone()
        } else if matches!(req.method, Method::POST | Method::PATCH | Method::PUT) {
            builder = builder.header("Content-Type", "application/json");
            Bytes::from_static(b"{}")
        } else {
            Bytes::new()
        };

        let hyper_req = builder
            .body(Full::new(body_bytes))
            .map_err(RestError::Build)?;

        tracing::debug!(method = %req.method, path = %req.path, "-> request");

        let sent = timeout(self.timeout, self.client.request(hyper_req)).await;
        let resp = match sent {
            Ok(Ok(resp)) => resp,
            Ok(Err(e)) => {
                return Err(e.into());
            }
            Err(_) => {
                return Err(RestError::Timeout);
            }
        };

        let status = resp.status();
        let headers = resp.headers().clone();
        let body_result = timeout(self.timeout, resp.into_body().collect()).await;
        let response_bytes = match body_result {
            Ok(Ok(collected)) => collected.to_bytes(),
            Ok(Err(e)) => {
                return Err(e.into());
            }
            Err(_) => {
                return Err(RestError::Timeout);
            }
        };

        let too_many_requests = (status == 429).then(|| parse_too_many_requests(&headers));
        // let info = ResponseInfo {
        //     bucket: parse_bucket_headers(&headers),
        //     too_many_requests: too_many_requests.clone(),
        // };

        self.emit(RestEvent::Response {
            route: req.route.clone(),
            method: req.method.clone(),
            status,
            body: response_bytes.clone(),
        });

        if let Some(too_many_requests) = too_many_requests {
            let retry_after_ms = too_many_requests.retry_after.as_millis() as u64;
            let data = Box::new(RateLimitData {
                global: too_many_requests.global,
                hash: header_str(&headers, "x-ratelimit-bucket").unwrap_or_default(),
                limit: header_i64(&headers, "x-ratelimit-limit").unwrap_or(-1),
                major_parameter: req.major_param.clone(),
                method: req.method.clone(),
                retry_after: retry_after_ms,
                route: req.route.clone(),
                scope: too_many_requests.scope,
                sublimit_id: None,
                sublimit_timeout: if too_many_requests.scope == RateLimitScope::Shared {
                    retry_after_ms
                } else {
                    0
                },
                time_to_reset: retry_after_ms,
            });

            self.emit(RestEvent::RateLimited(data.clone()));
            return Err(RestError::RateLimited(data));
        }

        if !status.is_success() {
            return Err(RestError::ApiError {
                status,
                body: response_bytes,
            });
        }

        Ok(response_bytes)
    }
}

fn header_str(headers: &HeaderMap, name: &str) -> Option<String> {
    headers.get(name)?.to_str().ok().map(str::to_string)
}
fn header_i64(headers: &HeaderMap, name: &str) -> Option<i64> {
    header_str(headers, name)?.parse().ok()
}
fn header_f64(headers: &HeaderMap, name: &str) -> Option<f64> {
    header_str(headers, name)?.parse().ok()
}

// fn parse_bucket_headers(headers: &HeaderMap) -> Option<BucketHeaders> {
//     Some(BucketHeaders {
//         hash: header_str(headers, "x-ratelimit-bucket")?,
//         limit: header_i64(headers, "x-ratelimit-limit").unwrap_or(-1),
//         remaining: header_i64(headers, "x-ratelimit-remaining").unwrap_or(-1),
//         reset_after: Duration::from_secs_f64(
//             header_f64(headers, "x-ratelimit-reset-after")
//                 .unwrap_or(0.0)
//                 .max(0.0),
//         ),
//         scope: header_str(headers, "x-ratelimit-scope")
//             .map(|s| RateLimitScope::parse(&s))
//             .unwrap_or_default(),
//     })
// }

fn parse_too_many_requests(headers: &HeaderMap) -> TooManyRequests {
    TooManyRequests {
        retry_after: Duration::from_secs_f64(
            header_f64(headers, "retry-after").unwrap_or(0.0).max(0.0),
        ),
        scope: header_str(headers, "x-ratelimit-scope")
            .map(|s| RateLimitScope::parse(&s))
            .unwrap_or_default(),
        global: headers.get("x-ratelimit-global").is_some(),
    }
}

/// Shorthand for arbitrary paths.
///
/// Always prefer a hand-written route constructor
/// + `execute` directly instead when one exists, for better bucket-inference accuracy (see
/// `Request::raw` docs).
impl Rest {
    /// `GET path`, deserializing the JSON response into `T`.
    pub async fn get<T: DeserializeOwned>(&self, path: impl Into<String>) -> Result<T, RestError> {
        self.execute(Request::raw(Method::GET, path)).await
    }

    /// `DELETE path`, deserializing the JSON response into `T` (often `()` via
    /// `serde_json::Value::Null` for endpoints that return 204 No Content).
    pub async fn delete<T: DeserializeOwned>(
        &self,
        path: impl Into<String>,
    ) -> Result<T, RestError> {
        self.execute(Request::raw(Method::DELETE, path)).await
    }

    /// `POST path` with a JSON body, deserializing the response into `T`.
    pub async fn post<T: DeserializeOwned>(
        &self,
        path: impl Into<String>,
        body: &impl serde::Serialize,
    ) -> Result<T, RestError> {
        self.execute(Request::raw(Method::POST, path).json(body))
            .await
    }

    /// `PATCH path` with a JSON body, deserializing the response into `T`.
    pub async fn patch<T: DeserializeOwned>(
        &self,
        path: impl Into<String>,
        body: &impl serde::Serialize,
    ) -> Result<T, RestError> {
        self.execute(Request::raw(Method::PATCH, path).json(body))
            .await
    }

    /// `PUT path` with a JSON body, deserializing the response into `T`.
    pub async fn put<T: DeserializeOwned>(
        &self,
        path: impl Into<String>,
        body: &impl serde::Serialize,
    ) -> Result<T, RestError> {
        self.execute(Request::raw(Method::PUT, path).json(body))
            .await
    }

    /// Convenience for using a webhook without needing a bot token.
    pub async fn execute_webhook(
        &self,
        webhook_id: u64,
        webhook_token: &str,
        payload: &impl serde::Serialize,
    ) -> Result<(), RestError> {
        let path = format!("/webhooks/{webhook_id}/{webhook_token}");
        let route = format!("{BURST_MAJOR_ID_KEY}:webhooks/:id/:token");
        let mut req = Request::post(route, path).json(payload);

        req.major_param = webhook_id.to_string();
        self.execute_raw(req).await?;

        Ok(())
    }
}
