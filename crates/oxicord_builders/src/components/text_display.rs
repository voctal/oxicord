use oxicord_api_types::v10::components::{ApiTextDisplayComponent, ComponentType};

/// Builder for text displays.
///
/// # Panics
///
/// [`TextDisplayBuilder::build`] will panic if no content was set.
#[derive(Debug)]
pub struct TextDisplayBuilder {
    id: Option<i32>,
    content: Option<String>,
}

impl TextDisplayBuilder {
    pub const fn new() -> Self {
        Self {
            id: None,
            content: None,
        }
    }

    /// Sets the `id`.
    pub fn id(mut self, id: Option<i32>) -> Self {
        self.id = id;
        self
    }

    /// Sets the `content`.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn build(self) -> ApiTextDisplayComponent {
        ApiTextDisplayComponent {
            component_type: ComponentType::TextDisplay,
            id: None,
            content: self.content.expect("Text display requires content"),
        }
    }
}

impl Default for TextDisplayBuilder {
    fn default() -> Self {
        Self::new()
    }
}
