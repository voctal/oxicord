use oxicord_api_types::v10::components::{ApiFileUploadComponent, ComponentType};

/// Builder for file uploads.
#[derive(Debug)]
pub struct FileUploadBuilder {
    id: Option<i32>,
    custom_id: String,
    min_values: Option<u8>,
    max_values: Option<u8>,
    required: Option<bool>,
    file_types: Option<Vec<String>>,
}

impl FileUploadBuilder {
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self {
            id: None,
            custom_id: custom_id.into(),
            min_values: None,
            max_values: None,
            required: None,
            file_types: None,
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

    /// Sets the `min_values`.
    pub fn min_values(mut self, min_values: u8) -> Self {
        self.min_values = Some(min_values);
        self
    }

    /// Sets the `max_values`.
    pub fn max_values(mut self, max_values: u8) -> Self {
        self.max_values = Some(max_values);
        self
    }

    /// Sets the `required` property.
    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    /// Sets the `file_types` filter.
    pub fn file_types(mut self, file_types: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.file_types = Some(file_types.into_iter().map(Into::into).collect());
        self
    }

    pub fn build(self) -> ApiFileUploadComponent {
        ApiFileUploadComponent {
            component_type: ComponentType::FileUpload,
            id: self.id,
            custom_id: self.custom_id,
            min_values: self.min_values,
            max_values: self.max_values,
            required: self.required,
            file_types: self.file_types,
        }
    }
}

impl Default for FileUploadBuilder {
    fn default() -> Self {
        Self::new(String::new())
    }
}
