//! Heuristic bucket-route inference, used by [`crate::Request::raw`] for paths that dont
//! have a constructor.
//!
//! Numeric path segments are normalized to `:id` so requests to the same resource share a
//! bucket.

const MAJOR_RESOURCES: [&str; 3] = ["channels", "guilds", "webhooks"];

/// Returns `(bucket_route, major_parameter)` for a path, e.g.
/// `/channels/123/messages/456` -> `("channels/123/messages/:id", "123")`.
pub(crate) fn infer_route(path: &str) -> (String, String) {
    let segments: Vec<&str> = path
        .trim_start_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();

    let major_index = segments.iter().enumerate().find_map(|(i, seg)| {
        if MAJOR_RESOURCES.contains(seg) {
            segments
                .get(i + 1)
                .filter(|next| is_snowflake(next))
                .map(|_| i + 1)
        } else {
            None
        }
    });

    let major_param = major_index
        .map(|idx| segments[idx].to_string())
        .unwrap_or_else(|| "global".to_string());

    let route = segments
        .iter()
        .enumerate()
        .map(|(i, seg)| {
            if Some(i) == major_index || !is_snowflake(seg) {
                (*seg).to_string()
            } else {
                ":id".to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("/");

    (route, major_param)
}

fn is_snowflake(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captures_channel_as_major_param() {
        let (route, major) = infer_route("/channels/123/messages/456");
        assert_eq!(route, "channels/123/messages/:id");
        assert_eq!(major, "123");
    }

    #[test]
    fn different_channels_get_different_routes() {
        let (a, _) = infer_route("/channels/111/messages/999");
        let (b, _) = infer_route("/channels/222/messages/999");
        assert_ne!(
            a, b,
            "different channels must not collapse into the same bucket route"
        );
    }

    #[test]
    fn no_major_resource_falls_back_to_global() {
        let (route, major) = infer_route("/users/@me");
        assert_eq!(route, "users/@me");
        assert_eq!(major, "global");
    }

    #[test]
    fn non_numeric_ids_pass_through_unchanged() {
        let (route, _) = infer_route("/webhooks/123/some-webhook-token/messages/@original");
        assert_eq!(route, "webhooks/123/some-webhook-token/messages/@original");
    }
}
