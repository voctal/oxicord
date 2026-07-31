use oxicord_api_types::v10::{
    components::{ApiButtonComponent, ButtonStyle, ComponentType},
    emoji::ApiMessageComponentEmoji,
};
use oxicord_snowflake::{EmojiId, SkuId};

/// Builder for buttons.
#[derive(Debug)]
pub struct ButtonBuilder {
    id: Option<i32>,
    style: ButtonStyle,
    label: Option<String>,
    emoji: Option<ApiMessageComponentEmoji>,
    custom_id: Option<String>,
    sku_id: Option<SkuId>,
    url: Option<String>,
    disabled: bool,
}

impl ButtonBuilder {
    pub const fn new() -> Self {
        Self {
            id: None,
            style: ButtonStyle::Primary,
            label: None,
            emoji: None,
            custom_id: None,
            sku_id: None,
            url: None,
            disabled: false,
        }
    }

    /// Sets the `id`.
    pub fn id(mut self, id: Option<i32>) -> Self {
        self.id = id;
        self
    }

    /// Sets the `style`.
    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    /// `style` helper.
    pub fn primary(mut self) -> Self {
        self.style = ButtonStyle::Primary;
        self
    }

    /// `style` helper.
    pub fn secondary(mut self) -> Self {
        self.style = ButtonStyle::Secondary;
        self
    }

    /// `style` helper.
    pub fn success(mut self) -> Self {
        self.style = ButtonStyle::Success;
        self
    }

    /// `style` helper.
    pub fn danger(mut self) -> Self {
        self.style = ButtonStyle::Danger;
        self
    }

    /// `style` helper.
    pub fn link(mut self, url: impl Into<String>) -> Self {
        self.style = ButtonStyle::Link;
        self.url = Some(url.into());
        self
    }

    /// `style` helper.
    pub fn premium(mut self, sku_id: impl Into<SkuId>) -> Self {
        self.style = ButtonStyle::Premium;
        self.sku_id = Some(sku_id.into());
        self
    }

    /// Sets the `label`.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
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

    /// Sets the `custom_id`. (cannot be used for link and premium buttons).
    pub fn custom_id(mut self, custom_id: impl Into<String>) -> Self {
        self.custom_id = Some(custom_id.into());
        self
    }

    /// Sets the `sku_id`. (only for premium buttons).
    pub fn sku_id(mut self, sku_id: impl Into<SkuId>) -> Self {
        self.sku_id = Some(sku_id.into());
        self
    }

    /// Sets the `url`. (only for link buttons).
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Sets the `disabled` property.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Build the component.
    pub fn build(self) -> ApiButtonComponent {
        ApiButtonComponent {
            component_type: ComponentType::Button,
            id: self.id,
            style: self.style,
            label: self.label,
            emoji: self.emoji,
            custom_id: self.custom_id,
            sku_id: self.sku_id,
            url: self.url,
            disabled: self.disabled,
        }
    }
}

impl Default for ButtonBuilder {
    fn default() -> Self {
        Self::new()
    }
}
