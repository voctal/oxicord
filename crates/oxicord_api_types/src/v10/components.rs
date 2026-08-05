use oxicord_macros::{discord_bitflags, discord_enum, discord_type};
use oxicord_snowflake::{AttachmentId, SkuId};

use crate::v10::emoji::ApiMessageComponentEmoji;

/// https://discord.com/developers/docs/components/reference#component-object-component-types
#[discord_enum(u8)]
pub enum ComponentType {
    ActionRow = 1,
    /// Button component.
    Button = 2,
    StringSelect = 3,
    /**
     * Text Input component.
     */
    TextInput = 4,
    /**
     * Select menu for users.
     */
    UserSelect = 5,
    /**
     * Select menu for roles.
     */
    RoleSelect = 6,
    /**
     * Select menu for mentionables (users and roles).
     */
    MentionableSelect = 7,
    /**
     * Select menu for channels.
     */
    ChannelSelect = 8,
    Section = 9,
    TextDisplay = 10,
    Thumbnail = 11,
    MediaGallery = 12,
    File = 13,
    Separator = 14,
    Container = 17,
    Label = 18,
    FileUpload = 19,
    RadioGroup = 21,
    CheckboxGroup = 22,
    Checkbox = 23,
}

/// An enum of all components.
///
/// TODO: less memory usage if possible
#[discord_type]
pub enum ApiComponent {
    ActionRow(ApiActionRowComponent),
    Button(ApiButtonComponent),
    StringSelect(ApiStringSelectComponent),
    TextDisplay(ApiTextDisplayComponent),
    Separator(ApiSeparatorComponent),
    Container(ApiContainerComponent),
    Thumbnail(ApiThumbnailComponent),
    File(ApiFileComponent),
}

/// https://discord.com/developers/docs/components/reference#button
#[discord_type]
pub struct ApiButtonComponent {
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    /// Optional identifier for the button.
    pub id: Option<i32>,
    pub style: ButtonStyle,
    pub label: Option<String>,
    pub emoji: Option<ApiMessageComponentEmoji>,
    pub custom_id: Option<String>,
    pub sku_id: Option<SkuId>,
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

/// https://discord.com/developers/docs/components/reference#action-row
#[discord_type]
pub struct ApiActionRowComponent {
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    pub id: Option<i32>,
    pub components: Vec<ApiComponent>,
}

/// https://discord.com/developers/docs/components/reference#separator
#[discord_type]
pub struct ApiSeparatorComponent {
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    pub id: Option<i32>,
    /// Whether extra vertical padding should be added.
    pub divider: bool,
    /// Spacing size around the separator.
    pub spacing: SeparatorSpacingSize,
}

#[discord_enum(u8)]
pub enum SeparatorSpacingSize {
    Small = 1,
    Large = 2,
}

/// https://discord.com/developers/docs/components/reference#container
#[discord_type]
pub struct ApiContainerComponent {
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    pub id: Option<i32>,
    pub components: Vec<ApiComponent>,
    /// Accent color shown on the container.
    pub accent_color: Option<Option<u32>>,
    /// Whether the container is rendered as a spoiler.
    pub spoiler: Option<bool>,
}

/// https://discord.com/developers/docs/components/reference#text-display
#[discord_type]
pub struct ApiTextDisplayComponent {
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    pub id: Option<i32>,
    /// Markdown text to display.
    pub content: String,
}

/// https://discord.com/developers/docs/components/reference#string-select
#[discord_type]
pub struct ApiStringSelectComponent {
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    pub id: Option<i32>,
    pub custom_id: String,
    pub options: Vec<ApiStringSelectOption>,
    pub placeholder: Option<String>,
    pub min_values: Option<u8>,
    pub max_values: Option<u8>,
    pub disabled: bool,
}

/// https://discord.com/developers/docs/components/reference#select-menu-object-select-option-structure
#[discord_type]
pub struct ApiStringSelectOption {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub emoji: Option<ApiMessageComponentEmoji>,
    pub default: bool,
}

/// <https://discord.com/developers/docs/components/reference#unfurled-media-item-structure>
#[discord_type]
pub struct ApiUnfurledMediaItem {
    /// Supports arbitrary urls and `attachment://<filename>` references.
    pub url: String,
    /// The proxied url of the media item.
    #[serde(default)]
    pub proxy_url: Option<String>,
    /// The height of the media item (if image or video).
    #[serde(default)]
    pub height: Option<u32>,
    /// The width of the media item (if image or video).
    #[serde(default)]
    pub width: Option<u32>,
    /// Thumbhash placeholder (if image or video).
    #[serde(default)]
    pub placeholder: Option<String>,
    /// Version of the placeholder (if image or video).
    #[serde(default)]
    pub placeholder_version: Option<u32>,
    /// The media type of the content.
    #[serde(default)]
    pub content_type: Option<String>,
    /// Unfurled media item flags combined as a bitfield.
    #[serde(default)]
    pub flags: Option<UnfurledMediaItemFlags>,
    /// The id of the uploaded attachment.
    #[serde(default)]
    pub attachment_id: Option<AttachmentId>,
}

discord_bitflags!(
    /// Flags for an unfurled media item.
    ///
    /// <https://discord.com/developers/docs/components/reference#unfurled-media-item-structure>
    pub struct UnfurledMediaItemFlags: u32 {
        /// This image is animated.
        const IS_ANIMATED = 1 << 0;
    }
);

/// <https://discord.com/developers/docs/components/reference#thumbnail>
#[discord_type]
pub struct ApiThumbnailComponent {
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    /// Optional identifier for the component.
    pub id: Option<i32>,
    /// The media for the thumbnail.
    pub media: ApiUnfurledMediaItem,
    /// Alt text for the media.
    pub description: Option<String>,
    /// Whether the thumbnail should be a spoiler (blurred out).
    pub spoiler: Option<bool>,
}

/// <https://docs.discord.com/developers/components/reference#file>
#[discord_type]
pub struct ApiFileComponent {
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    /// Optional identifier for the component.
    pub id: Option<i32>,
    /// The file referenced via an `attachment://<filename>` URI.
    pub file: ApiUnfurledMediaItem,
    /// Whether the file should be blurred as a spoiler. Defaults to `false`.
    pub spoiler: Option<bool>,
    /// The file's name (read-only).
    pub name: Option<String>,
    /// The file's size in bytes (read-only).
    pub size: Option<u32>,
}
