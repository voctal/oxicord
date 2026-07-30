use crate::VerifyError;

use ed25519_dalek::{Signature, Verifier as _, VerifyingKey};
use http::HeaderMap;

/// Discord Ed25519 signature header.
pub const SIGNATURE_HEADER: &str = "X-Signature-Ed25519";

/// Discord timestamp header.
pub const TIMESTAMP_HEADER: &str = "X-Signature-Timestamp";

/// Verifies Discord interaction requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Verifier {
    key: VerifyingKey,
}

impl Verifier {
    /// Creates a new verifier.
    pub const fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    /// Creates a verifier from a 32 bytes public key.
    pub fn from_bytes(bytes: [u8; 32]) -> Result<Self, VerifyError> {
        let key = VerifyingKey::from_bytes(&bytes).map_err(|_| VerifyError::InvalidPublicKey)?;

        Ok(Self::new(key))
    }

    /// Creates a verifier from a hex-encoded public key.
    pub fn from_hex(public_key: impl AsRef<str>) -> Result<Self, VerifyError> {
        let bytes = hex::decode(public_key.as_ref()).map_err(|_| VerifyError::InvalidPublicKey)?;

        let bytes: [u8; 32] = bytes
            .try_into()
            .map_err(|_| VerifyError::InvalidPublicKey)?;

        Self::from_bytes(bytes)
    }

    /// Returns the underlying public key.
    pub const fn public_key(&self) -> &VerifyingKey {
        &self.key
    }

    /// Verifies a request.
    pub fn verify(&self, headers: &HeaderMap, body: &[u8]) -> Result<(), VerifyError> {
        verify(&self.key, headers, body)
    }

    /// Verifies request parts.
    pub fn verify_parts(
        &self,
        signature: impl AsRef<str>,
        timestamp: impl AsRef<str>,
        body: &[u8],
    ) -> Result<(), VerifyError> {
        verify_parts(&self.key, signature, timestamp, body)
    }
}

/// Verifies an interaction request.
pub fn verify(key: &VerifyingKey, headers: &HeaderMap, body: &[u8]) -> Result<(), VerifyError> {
    let signature = headers
        .get(SIGNATURE_HEADER)
        .ok_or(VerifyError::MissingSignature)?
        .to_str()
        .map_err(|_| VerifyError::MissingSignature)?;

    let timestamp = headers
        .get(TIMESTAMP_HEADER)
        .ok_or(VerifyError::MissingTimestamp)?
        .to_str()
        .map_err(|_| VerifyError::MissingTimestamp)?;

    verify_parts(key, signature, timestamp, body)
}

/// Verifies request parts.
pub fn verify_parts(
    key: &VerifyingKey,
    signature: impl AsRef<str>,
    timestamp: impl AsRef<str>,
    body: &[u8],
) -> Result<(), VerifyError> {
    let signature =
        hex::decode(signature.as_ref()).map_err(|_| VerifyError::InvalidSignatureEncoding)?;

    let signature =
        Signature::from_slice(&signature).map_err(|_| VerifyError::InvalidSignatureEncoding)?;

    let timestamp = timestamp.as_ref();

    let mut message = Vec::with_capacity(timestamp.len() + body.len());

    message.extend_from_slice(timestamp.as_bytes());
    message.extend_from_slice(body);

    key.verify(&message, &signature)
        .map_err(|_| VerifyError::InvalidSignature)
}
