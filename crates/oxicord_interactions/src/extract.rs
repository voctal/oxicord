use crate::{ExtractError, Verifier};

use http::HeaderMap;
use serde::Deserialize;

/// Verify a request and deserialize JSON.
pub fn verify_and_parse<'de, T>(
    verifier: &Verifier,
    headers: &HeaderMap,
    body: &'de [u8],
) -> Result<T, ExtractError>
where
    T: Deserialize<'de>,
{
    verifier.verify(headers, body)?;

    Ok(serde_json::from_slice(body)?)
}
