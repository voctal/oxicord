use oxicord_api_types::v10::components::{
    ApiCheckboxGroupComponent, ApiCheckboxGroupOption, ComponentType,
};

/// Builder for checkbox group options.
#[derive(Debug)]
pub struct CheckboxGroupOptionBuilder {
    value: String,
    label: String,
    description: Option<String>,
    default: Option<bool>,
}

impl CheckboxGroupOptionBuilder {
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

    pub fn build(self) -> ApiCheckboxGroupOption {
        ApiCheckboxGroupOption {
            value: self.value,
            label: self.label,
            description: self.description,
            default: self.default,
        }
    }
}

impl Default for CheckboxGroupOptionBuilder {
    fn default() -> Self {
        Self::new(String::new(), String::new())
    }
}

/// Builder for checkbox groups.
#[derive(Debug)]
pub struct CheckboxGroupBuilder {
    id: Option<i32>,
    custom_id: String,
    options: Vec<ApiCheckboxGroupOption>,
    min_values: Option<u8>,
    max_values: Option<u8>,
    required: Option<bool>,
}

impl CheckboxGroupBuilder {
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self {
            id: None,
            custom_id: custom_id.into(),
            options: Vec::new(),
            min_values: None,
            max_values: None,
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

    /// Sets the `options`, replacing any previously set.
    pub fn options(mut self, options: Vec<ApiCheckboxGroupOption>) -> Self {
        self.options = options;
        self
    }

    /// Adds a single option to the group.
    pub fn add_option(mut self, option: ApiCheckboxGroupOption) -> Self {
        self.options.push(option);
        self
    }

    /// Sets the `min_values` property.
    pub fn min_values(mut self, min_values: u8) -> Self {
        self.min_values = Some(min_values);
        self
    }

    /// Sets the `max_values` property.
    pub fn max_values(mut self, max_values: u8) -> Self {
        self.max_values = Some(max_values);
        self
    }

    /// Sets whether a selection within the group is required.
    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    pub fn build(self) -> ApiCheckboxGroupComponent {
        ApiCheckboxGroupComponent {
            component_type: ComponentType::CheckboxGroup,
            id: self.id,
            custom_id: self.custom_id,
            options: self.options,
            min_values: self.min_values,
            max_values: self.max_values,
            required: self.required,
        }
    }
}

impl Default for CheckboxGroupBuilder {
    fn default() -> Self {
        Self::new(String::new())
    }
}
