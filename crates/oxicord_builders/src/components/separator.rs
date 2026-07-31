use oxicord_api_types::v10::components::{
    ApiSeparatorComponent, ComponentType, SeparatorSpacingSize,
};

/// Builder for action rows.
#[derive(Debug)]
pub struct SeparatorBuilder {
    id: Option<i32>,
    divider: Option<bool>,
    spacing: Option<SeparatorSpacingSize>,
}

impl SeparatorBuilder {
    pub const fn new() -> Self {
        Self {
            id: None,
            divider: None,
            spacing: None,
        }
    }

    /// Sets the `id`.
    pub fn id(mut self, id: Option<i32>) -> Self {
        self.id = id;
        self
    }

    /// Sets the `divider`.
    pub fn divider(mut self, divider: bool) -> Self {
        self.divider = Some(divider);
        self
    }

    /// Sets the `spacing`.
    pub fn spacing(mut self, spacing: SeparatorSpacingSize) -> Self {
        self.spacing = Some(spacing);
        self
    }

    /// Build the component.
    pub fn build(self) -> ApiSeparatorComponent {
        ApiSeparatorComponent {
            component_type: ComponentType::Separator,
            id: self.id,
            // TODO: the component should take Option for both
            divider: self.divider.unwrap_or(false),
            spacing: self.spacing.unwrap_or(SeparatorSpacingSize::Small),
        }
    }
}

impl Default for SeparatorBuilder {
    fn default() -> Self {
        Self::new()
    }
}
