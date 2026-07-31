use oxicord_api_types::v10::components::{ApiComponent, ApiContainerComponent, ComponentType};

use crate::{ActionRowBuilder, SeparatorBuilder, TextDisplayBuilder};

/// Builder for containers.
#[derive(Debug)]
pub struct ContainerBuilder {
    id: Option<i32>,
    components: Vec<ApiComponent>,
    accent_color: Option<u32>,
    spoiler: bool,
}

impl ContainerBuilder {
    pub const fn new() -> Self {
        Self {
            id: None,
            components: Vec::new(),
            accent_color: None,
            spoiler: false,
        }
    }

    /// Sets the `id`.
    pub fn id(mut self, id: Option<i32>) -> Self {
        self.id = id;
        self
    }

    pub fn add_text_display<F>(mut self, f: F) -> Self
    where
        F: FnOnce(TextDisplayBuilder) -> TextDisplayBuilder,
    {
        self.components.push(ApiComponent::TextDisplay(
            f(TextDisplayBuilder::new()).build(),
        ));
        self
    }

    pub fn add_separator<F>(mut self, f: F) -> Self
    where
        F: FnOnce(SeparatorBuilder) -> SeparatorBuilder,
    {
        self.components
            .push(ApiComponent::Separator(f(SeparatorBuilder::new()).build()));
        self
    }

    pub fn add_action_row<F>(mut self, f: F) -> Self
    where
        F: FnOnce(ActionRowBuilder) -> ActionRowBuilder,
    {
        self.components
            .push(ApiComponent::ActionRow(f(ActionRowBuilder::new()).build()));
        self
    }

    /// Sets the `accent_color`.
    pub fn accent_color(mut self, color: u32) -> Self {
        self.accent_color = Some(color);
        self
    }

    /// Sets the `spoiler`.
    pub fn spoiler(mut self, spoiler: bool) -> Self {
        self.spoiler = spoiler;
        self
    }

    /// Build the component.
    pub fn build(self) -> ApiContainerComponent {
        ApiContainerComponent {
            component_type: ComponentType::Container,
            id: None,
            accent_color: if self.accent_color.is_none() {
                None
            } else {
                Some(self.accent_color) // Some(Some(_))
            },
            components: self.components,
            spoiler: Some(self.spoiler),
        }
    }
}

impl Default for ContainerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ContainerBuilder> for Vec<ApiComponent> {
    fn from(builder: ContainerBuilder) -> Self {
        vec![ApiComponent::Container(builder.build())]
    }
}
