use oxicord_api_types::v10::components::{ApiComponent, ApiLabelComponent, ComponentType};

/// Builder for labels.
#[derive(Debug)]
pub struct LabelBuilder {
    id: Option<i32>,
    label: String,
    description: Option<String>,
    component: Box<ApiComponent>,
}

impl LabelBuilder {
    pub fn new(label: impl Into<String>, component: impl Into<Box<ApiComponent>>) -> Self {
        Self {
            id: None,
            label: label.into(),
            description: None,
            component: component.into(),
        }
    }

    /// Sets the `id`.
    pub fn id(mut self, id: Option<i32>) -> Self {
        self.id = id;
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

    /// Sets the `component`.
    pub fn component(mut self, component: impl Into<Box<ApiComponent>>) -> Self {
        self.component = component.into();
        self
    }

    /// Build the component.
    pub fn build(self) -> ApiLabelComponent {
        ApiLabelComponent {
            component_type: ComponentType::Label,
            id: self.id,
            label: self.label,
            description: self.description,
            component: self.component,
        }
    }
}
