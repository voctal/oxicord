use oxicord_api_types::v10::components::{
    ApiThumbnailComponent, ApiUnfurledMediaItem, ComponentType,
};

/// Builder for thumbnails.
#[derive(Debug)]
pub struct ThumbnailBuilder {
    id: Option<i32>,
    url: String,
    description: Option<String>,
    spoiler: Option<bool>,
}

impl ThumbnailBuilder {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            id: None,
            url: url.into(),
            description: None,
            spoiler: None,
        }
    }

    /// Sets the `id`.
    pub fn id(mut self, id: Option<i32>) -> Self {
        self.id = id;
        self
    }

    /// Sets the media `url`.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    /// Sets the `description`.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the `spoiler` property.
    pub fn spoiler(mut self, spoiler: bool) -> Self {
        self.spoiler = Some(spoiler);
        self
    }

    pub fn build(self) -> ApiThumbnailComponent {
        ApiThumbnailComponent {
            component_type: ComponentType::Thumbnail,
            id: self.id,
            media: ApiUnfurledMediaItem {
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
            description: self.description,
            spoiler: self.spoiler,
        }
    }
}
