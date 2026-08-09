use oxicord_api_types::v10::components::{
    ApiRadioGroupComponent, ApiRadioGroupOption, ComponentType,
};

/// Builder for radio group options.
#[derive(Debug)]
pub struct RadioGroupOptionBuilder {
    value: String,
    label: String,
    description: Option<String>,
    default: Option<bool>,
}

impl RadioGroupOptionBuilder {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            default: None,
        }
    }

    /// Sets the `value`.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    /// Sets the `label`.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Sets the `description`.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the `default` property.
    pub fn default(mut self, default: bool) -> Self {
        self.default = Some(default);
        self
    }

    pub fn build(self) -> ApiRadioGroupOption {
        ApiRadioGroupOption {
            value: self.value,
            label: self.label,
            description: self.description,
            default: self.default,
        }
    }
}

impl Default for RadioGroupOptionBuilder {
    fn default() -> Self {
        Self::new(String::new(), String::new())
    }
}

/// Builder for radio groups.
#[derive(Debug)]
pub struct RadioGroupBuilder {
    id: Option<i32>,
    custom_id: String,
    options: Vec<ApiRadioGroupOption>,
    required: Option<bool>,
}

impl RadioGroupBuilder {
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self {
            id: None,
            custom_id: custom_id.into(),
            options: Vec::new(),
            required: None,
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

    /// Sets the `options`.
    pub fn options(mut self, options: Vec<ApiRadioGroupOption>) -> Self {
        self.options = options;
        self
    }

    /// Adds a single option to the group.
    pub fn add_option<F>(mut self, f: F) -> Self
    where
        F: FnOnce(RadioGroupOptionBuilder) -> RadioGroupOptionBuilder,
    {
        self.options
            // TODO: remove cast once `default` setter is renamed
            .push(f(<RadioGroupOptionBuilder as Default>::default()).build());
        self
    }

    /// Sets whether a selection within the group is required.
    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    pub fn build(self) -> ApiRadioGroupComponent {
        ApiRadioGroupComponent {
            component_type: ComponentType::RadioGroup,
            id: self.id,
            custom_id: self.custom_id,
            options: self.options,
            required: self.required,
        }
    }
}

impl Default for RadioGroupBuilder {
    fn default() -> Self {
        Self::new(String::new())
    }
}
