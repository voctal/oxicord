use oxicord_api_types::v10::components::{ApiTextInputComponent, ComponentType, TextInputStyle};

/// Builder for text inputs.
#[derive(Debug)]
pub struct TextInputBuilder {
    id: Option<i32>,
    custom_id: String,
    style: TextInputStyle,
    min_length: Option<u16>,
    max_length: Option<u16>,
    required: Option<bool>,
    value: Option<String>,
    placeholder: Option<String>,
}

impl TextInputBuilder {
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self {
            id: None,
            custom_id: custom_id.into(),
            style: TextInputStyle::Short,
            min_length: None,
            max_length: None,
            required: None,
            value: None,
            placeholder: None,
        }
    }

    /// Sets the `id`.
    pub fn id(mut self, id: i32) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the `style`.
    pub fn style(mut self, style: TextInputStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the `min_length`.
    pub fn min_length(mut self, min_length: u16) -> Self {
        self.min_length = Some(min_length);
        self
    }

    /// Sets the `max_length`.
    pub fn max_length(mut self, max_length: u16) -> Self {
        self.max_length = Some(max_length);
        self
    }

    /// Sets the `required` flag.
    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    /// Sets the `value`.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Sets the `placeholder`.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Builds the `ApiTextInputComponent`.
    pub fn build(self) -> ApiTextInputComponent {
        ApiTextInputComponent {
            component_type: ComponentType::TextInput,
            id: self.id,
            custom_id: self.custom_id,
            style: self.style,
            min_length: self.min_length,
            max_length: self.max_length,
            required: self.required,
            value: self.value,
            placeholder: self.placeholder,
        }
    }
}

impl Default for TextInputBuilder {
    fn default() -> Self {
        Self {
            id: None,
            custom_id: String::new(),
            style: TextInputStyle::Short,
            min_length: None,
            max_length: None,
            required: None,
            value: None,
            placeholder: None,
        }
    }
}
