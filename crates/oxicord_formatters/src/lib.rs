//! # oxicord_formatters
//!
//! Helpers for building Discord message markdown, mentions, timestamps
//! and emoji strings.
//!
//! ```
//! use oxicord_formatters::*;
//!
//! let msg = format!(
//!     "{} welcome to {}! {}",
//!     user_mention(123456789012345678),
//!     channel_mention(123456789012345678),
//!     bold("enjoy your stay"),
//! );
//! assert_eq!(msg, "<@123456789012345678> welcome to <#123456789012345678>! **enjoy your stay**");
//! ```

mod links;
mod markdown;
mod mentions;
mod navigation;
mod timestamp;

pub use links::*;
pub use markdown::*;
pub use mentions::*;
pub use navigation::*;
pub use timestamp::*;
