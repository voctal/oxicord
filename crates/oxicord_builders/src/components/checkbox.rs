use oxicord_api_types::v10::components::{ApiCheckboxComponent, ComponentType};

/// Builder for checkboxes.
#[derive(Debug)]
pub struct CheckboxBuilder {
    id: Option<i32>,
    custom_id: String,
    default: Option<bool>,
}

impl CheckboxBuilder {
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self {
            id: None,
            custom_id: custom_id.into(),
            default: None,
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

    /// Sets whether the checkbox is checked by default.
    pub fn default(mut self, default: bool) -> Self {
        self.default = Some(default);
        self
    }

    pub fn build(self) -> ApiCheckboxComponent {
        ApiCheckboxComponent {
            component_type: ComponentType::Checkbox,
            id: self.id,
            custom_id: self.custom_id,
            default: self.default,
        }
    }
}

impl Default for CheckboxBuilder {
    fn default() -> Self {
        Self {
            id: None,
            custom_id: String::new(),
            default: None,
        }
    }
}
