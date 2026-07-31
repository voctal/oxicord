use oxicord_macros::discord_type;

use crate::Snowflake;

/// A partial emoji, used in some components.
///
/// `id` is None for unicode emojis, and
/// `name` can be None for custom emojis.
#[discord_type]
pub struct ApiMessageComponentEmoji {
    /// Emoji id
    pub id: Option<Snowflake>,
    /// Emoji name
    pub name: Option<String>,
    /// Whether this emoji is animated
    #[serde(default)]
    pub animated: Option<bool>,
}
