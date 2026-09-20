use std::sync::Arc;

use bytes::Bytes;
use hyper::Method;
use serde::Serialize;
use urlencoding::encode;

/// A request's data to the Discord API.
#[derive(Clone, Debug)]
pub struct Request {
    pub method: Method,
    /// The path appended to the API base URL, e.g. `/channels/123/messages/456`.
    pub path: String,
    /// The rate-limit bucket route template, e.g. `channels/123/messages/:id`.
    pub route: String,
    /// The major parameter value for this route (a channel/guild/webhook id, or
    /// `"global"` for routes with none, e.g. `/gateway`).
    pub major_param: String,
    pub body: Option<Bytes>,
    /// Set by [`Request::json`] if serialization failed.
    pub(crate) body_error: Option<Arc<serde_json::Error>>,
    pub audit_log_reason: Option<String>,
    /// Query string params, in insertion order.
    ///
    /// Not a map because the same key can be added more than once.
    pub query: Vec<(String, String)>,
}

impl Request {
    pub fn new(method: Method, route: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            method,
            path: path.into(),
            route: route.into(),
            major_param: "global".to_string(),
            body: None,
            body_error: None,
            audit_log_reason: None,
            query: Vec::new(),
        }
    }

    pub fn get(route: impl Into<String>, path: impl Into<String>) -> Self {
        Self::new(Method::GET, route, path)
    }
    pub fn post(route: impl Into<String>, path: impl Into<String>) -> Self {
        Self::new(Method::POST, route, path)
    }
    pub fn patch(route: impl Into<String>, path: impl Into<String>) -> Self {
        Self::new(Method::PATCH, route, path)
    }
    pub fn put(route: impl Into<String>, path: impl Into<String>) -> Self {
        Self::new(Method::PUT, route, path)
    }
    pub fn delete(route: impl Into<String>, path: impl Into<String>) -> Self {
        Self::new(Method::DELETE, route, path)
    }

    /// Sets the major parameter.
    ///
    /// Only needed when building a custom [`Request`]. The verb constructors
    /// default it to `"global"` and [`Request::raw`] infers it automatically.
    pub fn major_param(mut self, id: impl std::fmt::Display) -> Self {
        self.major_param = id.to_string();
        self
    }

    /// Builds a request for any path.
    ///
    /// The bucket route and major parameter are inferred heuristically (see the
    /// `routing::infer` module).
    pub fn raw(method: Method, path: impl Into<String>) -> Self {
        let path = path.into();
        let (route, major_param) = crate::routing::infer_route(&path);
        let mut req = Self::new(method, route, path);
        req.major_param = major_param;
        req
    }

    /// Attaches a JSON body. If serialization fails, the error is
    /// thrown by `Rest::execute`/`execute_raw` before sending.
    pub fn json<T: Serialize>(mut self, body: &T) -> Self {
        match serde_json::to_vec(body) {
            Ok(bytes) => self.body = Some(Bytes::from(bytes)),
            Err(e) => self.body_error = Some(Arc::new(e)),
        }
        self
    }

    /// Sets the `X-Audit-Log-Reason` header for this request.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.audit_log_reason = Some(reason.into());
        self
    }

    /// Adds a query string parameter. Can be called more than once with the same `key`.
    pub fn query(mut self, key: impl Into<String>, value: impl ToString) -> Self {
        self.query.push((key.into(), value.to_string()));
        self
    }

    pub(crate) fn full_path(&self) -> String {
        if self.query.is_empty() {
            return self.path.clone();
        }
        let qs = self
            .query
            .iter()
            .map(|(k, v)| format!("{k}={}", encode(v)))
            .collect::<Vec<_>>()
            .join("&");
        format!("{}?{qs}", self.path)
    }
}
