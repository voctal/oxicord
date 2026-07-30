use thiserror::Error;

/// A verification error.
#[derive(Debug, Error)]
pub enum VerifyError {
    #[error("missing X-Signature-Ed25519 header")]
    MissingSignature,

    #[error("missing X-Signature-Timestamp header")]
    MissingTimestamp,

    #[error("invalid public key")]
    InvalidPublicKey,

    #[error("invalid signature encoding")]
    InvalidSignatureEncoding,

    #[error("signature verification failed")]
    InvalidSignature,
}

#[derive(Debug, Error)]
pub enum ExtractError {
    #[error(transparent)]
    Verify(#[from] VerifyError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
