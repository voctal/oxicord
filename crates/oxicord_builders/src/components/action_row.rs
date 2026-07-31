use oxicord_api_types::v10::components::{ApiActionRowComponent, ApiComponent, ComponentType};

use crate::{ButtonBuilder, StringSelectBuilder};

/// Builder for action rows.
#[derive(Debug)]
pub struct ActionRowBuilder {
    id: Option<i32>,
    components: Vec<ApiComponent>,
}

impl ActionRowBuilder {
    pub const fn new() -> Self {
        Self {
            id: None,
            components: Vec::new(),
        }
    }

    /// Sets the `id`.
    pub fn id(mut self, id: Option<i32>) -> Self {
        self.id = id;
        self
    }

    pub fn add_button<F>(mut self, f: F) -> Self
    where
        F: FnOnce(ButtonBuilder) -> ButtonBuilder,
    {
        self.components
            .push(ApiComponent::Button(f(ButtonBuilder::new()).build()));
        self
    }

    pub fn add_string_select<F>(mut self, f: F) -> Self
    where
        F: FnOnce(StringSelectBuilder) -> StringSelectBuilder,
    {
        self.components.push(ApiComponent::StringSelect(
            f(StringSelectBuilder::new()).build(),
        ));
        self
    }

    /// Build the component.
    pub fn build(self) -> ApiActionRowComponent {
        ApiActionRowComponent {
            component_type: ComponentType::ActionRow,
            id: self.id,
            components: self.components,
        }
    }
}

impl Default for ActionRowBuilder {
    fn default() -> Self {
        Self::new()
    }
}
