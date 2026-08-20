use std::ops::RangeBounds;

use oxicord_api_types::v10::components::{
    ApiMediaGalleryComponent, ApiMediaGalleryItem, ApiUnfurledMediaItem, ComponentType,
};

/// Builder for media gallery items.
#[derive(Debug)]
pub struct MediaGalleryItemBuilder {
    url: String,
    description: Option<String>,
    spoiler: Option<bool>,
}

impl MediaGalleryItemBuilder {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            description: None,
            spoiler: None,
        }
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

    pub fn build(self) -> ApiMediaGalleryItem {
        ApiMediaGalleryItem {
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

impl Default for MediaGalleryItemBuilder {
    fn default() -> Self {
        Self::new(String::new())
    }
}

/// Builder for media galleries.
#[derive(Debug)]
pub struct MediaGalleryBuilder {
    id: Option<i32>,
    items: Vec<ApiMediaGalleryItem>,
}

impl MediaGalleryBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            items: Vec::new(),
        }
    }

    /// Sets the `id`.
    pub fn id(mut self, id: Option<i32>) -> Self {
        self.id = id;
        self
    }

    /// Adds a single item to the gallery (max 10).
    pub fn add_item<F>(mut self, f: F) -> Self
    where
        F: FnOnce(MediaGalleryItemBuilder) -> MediaGalleryItemBuilder,
    {
        self.items
            .push(f(MediaGalleryItemBuilder::default()).build());
        self
    }

    /// Removes, replaces, or inserts media gallery items in range, like Vec::splice.
    pub fn splice_items<R>(
        mut self,
        range: R,
        replace_with: impl IntoIterator<Item = ApiMediaGalleryItem>,
    ) -> Self
    where
        R: RangeBounds<usize>,
    {
        self.items.splice(range, replace_with);
        self
    }

    pub fn build(self) -> ApiMediaGalleryComponent {
        ApiMediaGalleryComponent {
            component_type: ComponentType::MediaGallery,
            id: self.id,
            items: self.items,
        }
    }
}

impl Default for MediaGalleryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
