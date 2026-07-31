use oxicord_macros::{discord_str_enum, discord_type};

/// <https://discord.com/developers/docs/resources/message#embed-object-embed-types>
#[discord_str_enum]
pub enum EmbedType {
    Rich,
    Image,
    Video,
    Gifv,
    Article,
    Link,
    PollResult,
}

#[discord_type]
pub struct ApiEmbedFooter {
    pub text: String,
    #[serde(default)]
    pub icon_url: Option<String>,
    #[serde(default)]
    pub proxy_icon_url: Option<String>,
}

#[discord_type]
pub struct ApiEmbedImage {
    pub url: String,
    #[serde(default)]
    pub proxy_url: Option<String>,
    #[serde(default)]
    pub height: Option<u32>,
    #[serde(default)]
    pub width: Option<u32>,
}

#[discord_type]
pub struct ApiEmbedThumbnail {
    pub url: String,
    #[serde(default)]
    pub proxy_url: Option<String>,
    #[serde(default)]
    pub height: Option<u32>,
    #[serde(default)]
    pub width: Option<u32>,
}

#[discord_type]
pub struct ApiEmbedVideo {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub proxy_url: Option<String>,
    #[serde(default)]
    pub height: Option<u32>,
    #[serde(default)]
    pub width: Option<u32>,
}

#[discord_type]
pub struct ApiEmbedProvider {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

#[discord_type]
pub struct ApiEmbedAuthor {
    pub name: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub icon_url: Option<String>,
    #[serde(default)]
    pub proxy_icon_url: Option<String>,
}

#[discord_type]
pub struct ApiEmbedField {
    pub name: String,
    pub value: String,
    #[serde(default)]
    pub inline: Option<bool>,
}

/// <https://discord.com/developers/docs/resources/message#embed-object>
#[discord_type]
pub struct ApiEmbed {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default, rename = "type")]
    pub embed_type: Option<EmbedType>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(default)]
    pub color: Option<u32>,
    #[serde(default)]
    pub footer: Option<ApiEmbedFooter>,
    #[serde(default)]
    pub image: Option<ApiEmbedImage>,
    #[serde(default)]
    pub thumbnail: Option<ApiEmbedThumbnail>,
    #[serde(default)]
    pub video: Option<ApiEmbedVideo>,
    #[serde(default)]
    pub provider: Option<ApiEmbedProvider>,
    /// Author information
    #[serde(default)]
    pub author: Option<ApiEmbedAuthor>,
    #[serde(default)]
    pub fields: Option<Vec<ApiEmbedField>>,
}
