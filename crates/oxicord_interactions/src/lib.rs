#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! HTTP utilities for handling Discord Interactions.

mod error;
mod extract;
mod verify;

pub use error::*;
pub use extract::*;
pub use verify::*;

// #[cfg(feature = "axum")]
// pub mod axum;
