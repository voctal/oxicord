use oxicord_macros::{discord_enum, discord_type};

/// https://discord.com/developers/docs/components/reference#button
#[discord_type]
#[derive(Hash)]
pub struct Button {
    pub r#type: u8,
    /// Optional identifier for the button.
    pub id: Option<i32>,
    pub style: ButtonStyle,
    pub label: Option<String>,
    // pub emoji: Option<>,
    pub custom_id: Option<String>,
    pub sku_id: Option<u64>,
    pub url: Option<String>,
    /// Whether the button is disabled.
    pub disabled: bool,
}

/// Button styles.
#[discord_enum(u8)]
#[derive(Hash)]
pub enum ButtonStyle {
    /// The most important or recommended action in a group of options.
    Primary = 1,
    /// Alternative or supporting actions.
    Secondary = 2,
    /// Positive confirmation or completion actions.
    Success = 3,
    /// An action with irreversible consequences.
    Danger = 4,
    /// Navigates to a URL.
    Link = 5,
    /// Purchase.
    Premium = 6,
}
