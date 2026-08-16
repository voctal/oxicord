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
    FileUpload(ApiFileUploadComponent),
    Checkbox(ApiCheckboxComponent),
    CheckboxGroup(ApiCheckboxGroupComponent),
    RadioGroup(ApiRadioGroupComponent),
    Label(ApiLabelComponent),
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
    /// Custom identifier for the select menu, max 100 characters.
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

/// <https://docs.discord.com/developers/components/reference#file-upload>
#[discord_type]
pub struct ApiFileUploadComponent {
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    /// Optional identifier for the component.
    pub id: Option<i32>,
    /// Custom identifier for the file upload, max 100 characters.
    pub custom_id: String,
    /// Minimum number of items that must be uploaded (defaults to 1); min 0, max 10
    pub min_values: Option<u8>,
    /// Maximum number of items that can be uploaded (defaults to 1); max 10
    pub max_values: Option<u8>,
    /// Whether at least one file is needed to submit the modal (defaults to `true`).
    pub required: Option<bool>,
    /// Restricts which files can be picked. See <https://docs.discord.com/developers/reference#file-type-filtering> for valid types.
    pub file_types: Option<Vec<String>>,
}

/// <https://docs.discord.com/developers/components/reference#checkbox>
#[discord_type]
pub struct ApiCheckboxComponent {
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    /// Optional identifier for the component.
    pub id: Option<i32>,
    /// Custom identifier for the checkbox, max 100 characters.
    pub custom_id: String,
    /// Whether the checkbox should be checked by default.
    pub default: Option<bool>,
}

/// <https://docs.discord.com/developers/components/reference#checkbox-group>
#[discord_type]
pub struct ApiCheckboxGroupComponent {
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    /// Optional identifier for the component.
    pub id: Option<i32>,
    /// Custom identifier for the checkbox group, max 100 characters.
    pub custom_id: String,
    /// List of options to display, between 1 and 10.
    pub options: Vec<ApiCheckboxGroupOption>,
    /// Minimum number of items that must be selected, between 0 and 10.
    pub min_values: Option<u8>,
    /// Maximum number of items that can be selected, between 1 and 10.
    pub max_values: Option<u8>,
    /// Whether a selection is required within the group.
    pub required: Option<bool>,
}

/// <https://docs.discord.com/developers/components/reference#checkbox-group-option-structure>
#[discord_type]
pub struct ApiCheckboxGroupOption {
    /// Developer-defined value for this option, up to 100 characters.
    pub value: String,
    /// Label displayed to the user, up to 100 characters.
    pub label: String,
    /// Optional description shown alongside the option, up to 100 characters.
    pub description: Option<String>,
    /// Whether this option is selected by default.
    pub default: Option<bool>,
}

/// <https://docs.discord.com/developers/components/reference#radio-group>
#[discord_type]
pub struct ApiRadioGroupComponent {
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    /// Optional id for the component.
    pub id: Option<i32>,
    /// Custom identifier for the radio group, max 100 characters.
    pub custom_id: String,
    /// List of options to display, between 2 and 10.
    pub options: Vec<ApiRadioGroupOption>,
    /// Whether a selection is required.
    pub required: Option<bool>,
}

/// <https://docs.discord.com/developers/components/reference#radio-group-option-structure>
#[discord_type]
pub struct ApiRadioGroupOption {
    /// Dev-defined value tied to this option, max 100 characters.
    pub value: String,
    /// Label displayed to the user, max 100 characters.
    pub label: String,
    /// Optional description shown alongside the option, max 100 characters.
    pub description: Option<String>,
    /// Whether this option is selected by default.
    pub default: Option<bool>,
}

/// <https://docs.discord.com/developers/components/reference#label>
#[discord_type]
pub struct ApiLabelComponent {
    /// The type of component.
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    /// Optional identifier for the component.
    pub id: Option<i32>,
    /// The label text.
    pub label: String,
    /// Optional description shown below the label.
    pub description: Option<String>,
    /// The wrapped component. See <https://docs.discord.com/developers/components/reference#label-label-child-components> for valid types.
    pub component: Box<ApiComponent>,
}
