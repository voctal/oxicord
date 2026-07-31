use oxicord_api_types::v10::{
    components::{ApiStringSelectComponent, ApiStringSelectOption, ComponentType},
    emoji::ApiMessageComponentEmoji,
};
use oxicord_snowflake::EmojiId;

/// Builder for string select menus.
#[derive(Debug)]
pub struct StringSelectBuilder {
    id: Option<i32>,
    custom_id: String,
    options: Vec<ApiStringSelectOption>,
    placeholder: Option<String>,
    min_values: Option<u8>,
    max_values: Option<u8>,
    disabled: bool,
}

impl StringSelectBuilder {
    pub const fn new() -> Self {
        Self {
            id: None,
            custom_id: String::new(),
            options: Vec::new(),
            placeholder: None,
            min_values: None,
            max_values: None,
            disabled: false,
        }
    }

    /// Sets the `id`.
    pub fn id(mut self, id: Option<i32>) -> Self {
        self.id = id;
        self
    }

    /// Sets the `custom_id`.
    pub fn custom_id(mut self, custom_id: impl Into<String>) -> Self {
        self.custom_id = custom_id.into();
        self
    }

    pub fn add_option<F>(mut self, f: F) -> Self
    where
        F: FnOnce(StringSelectOptionBuilder) -> StringSelectOptionBuilder,
    {
        self.options
            .push(f(StringSelectOptionBuilder::new()).build());
        self
    }

    /// Sets the `placeholder`.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Sets the `min_values`.
    pub fn min_values(mut self, min: u8) -> Self {
        self.min_values = Some(min);
        self
    }

    /// Sets the `max_values`.
    pub fn max_values(mut self, max: u8) -> Self {
        self.max_values = Some(max);
        self
    }

    /// Sets the `disabled`.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Build the component.
    pub fn build(self) -> ApiStringSelectComponent {
        ApiStringSelectComponent {
            component_type: ComponentType::StringSelect,
            id: self.id,
            custom_id: self.custom_id,
            options: self.options,
            placeholder: self.placeholder,
            min_values: self.min_values,
            max_values: self.max_values,
            disabled: self.disabled,
        }
    }
}

impl Default for StringSelectBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for string select menu options.
#[derive(Debug)]
pub struct StringSelectOptionBuilder {
    label: String,
    value: String,
    description: Option<String>,
    emoji: Option<ApiMessageComponentEmoji>,
    default: bool,
}

impl StringSelectOptionBuilder {
    pub const fn new() -> Self {
        Self {
            label: String::new(),
            value: String::new(),
            description: None,
            emoji: None,
            default: false,
        }
    }

    /// Sets the `label`.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Sets the `value`.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    /// Sets the `description`.
    pub fn description(mut self, description: Option<impl Into<String>>) -> Self {
        self.description = description.map(|d| d.into());
        self
    }

    /// Sets the `emoji`.
    ///
    /// Helpers:
    /// - [`Self::unicode_emoji`]
    /// - [`Self::custom_emoji`]
    pub fn emoji(mut self, emoji: Option<ApiMessageComponentEmoji>) -> Self {
        self.emoji = emoji;
        self
    }

    /// Sets the `emoji` as an unicode emoji.
    pub fn unicode_emoji(mut self, name: impl Into<String>) -> Self {
        self.emoji = Some(ApiMessageComponentEmoji {
            id: None,
            name: Some(name.into()),
            animated: None,
        });
        self
    }

    /// Sets the `emoji` as a custom emoji.
    pub fn custom_emoji(mut self, id: EmojiId) -> Self {
        self.emoji = Some(ApiMessageComponentEmoji {
            id: Some(id),
            name: None,
            animated: None,
        });
        self
    }

    /// Sets the `default`.
    pub fn default(mut self, default: bool) -> Self {
        self.default = default;
        self
    }

    /// Build the component.
    pub fn build(self) -> ApiStringSelectOption {
        ApiStringSelectOption {
            label: self.label,
            value: self.value,
            description: self.description,
            emoji: self.emoji,
            default: self.default,
        }
    }
}

impl Default for StringSelectOptionBuilder {
    fn default() -> Self {
        Self::new()
    }
}
