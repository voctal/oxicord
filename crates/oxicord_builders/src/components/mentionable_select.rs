use oxicord_api_types::v10::components::{
    ApiMentionableSelectComponent, ApiSelectDefaultValue, ComponentType,
};

/// Builder for mentionable selects.
#[derive(Debug)]
pub struct MentionableSelectBuilder {
    id: Option<i32>,
    custom_id: String,
    placeholder: Option<String>,
    default_values: Option<Vec<ApiSelectDefaultValue>>,
    min_values: Option<u8>,
    max_values: Option<u8>,
    required: Option<bool>,
    disabled: Option<bool>,
}

impl MentionableSelectBuilder {
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self {
            id: None,
            custom_id: custom_id.into(),
            placeholder: None,
            default_values: None,
            min_values: None,
            max_values: None,
            required: None,
            disabled: None,
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

    /// Sets the `placeholder`.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Sets the `default_values`.
    pub fn default_values(mut self, default_values: Vec<ApiSelectDefaultValue>) -> Self {
        self.default_values = Some(default_values);
        self
    }

    /// Sets the `min_values`.
    pub fn min_values(mut self, min_values: u8) -> Self {
        self.min_values = Some(min_values);
        self
    }

    /// Sets the `max_values`.
    pub fn max_values(mut self, max_values: u8) -> Self {
        self.max_values = Some(max_values);
        self
    }

    /// Sets the `required`.
    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    /// Sets the `disabled`.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    /// Build the component.
    pub fn build(self) -> ApiMentionableSelectComponent {
        ApiMentionableSelectComponent {
            component_type: ComponentType::MentionableSelect,
            id: self.id,
            custom_id: self.custom_id,
            placeholder: self.placeholder,
            default_values: self.default_values,
            min_values: self.min_values,
            max_values: self.max_values,
            required: self.required,
            disabled: self.disabled,
        }
    }
}
