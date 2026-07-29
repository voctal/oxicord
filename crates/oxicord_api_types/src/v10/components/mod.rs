mod button;

use oxicord_macros::discord_enum;

pub use button::*;

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
