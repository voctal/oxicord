use oxicord_api_types::v10::components::{ApiFileComponent, ApiUnfurledMediaItem, ComponentType};

/// Builder for files.
#[derive(Debug)]
pub struct FileBuilder {
    id: Option<i32>,
    url: String,
    spoiler: Option<bool>,
}

impl FileBuilder {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            id: None,
            url: url.into(),
            spoiler: None,
        }
    }

    /// Sets the `id`.
    pub fn id(mut self, id: Option<i32>) -> Self {
        self.id = id;
        self
    }

    /// Sets the file `url`.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    /// Sets the `spoiler`.
    pub fn spoiler(mut self, spoiler: bool) -> Self {
        self.spoiler = Some(spoiler);
        self
    }

    pub fn build(self) -> ApiFileComponent {
        ApiFileComponent {
            component_type: ComponentType::File,
            id: self.id,
            file: ApiUnfurledMediaItem {
                url: self.url,
                proxy_url: None,
                height: None,
                width: None,
                placeholder: None,
                placeholder_version: None,
                content_type: None,
                flags: None,
                attachment_id: None,
            },
            spoiler: self.spoiler,
            name: None,
            size: None,
        }
    }
}
