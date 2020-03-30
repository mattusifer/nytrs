use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct Response<A: Article> {
    pub status: String,
    pub copyright: String,
    pub num_results: Option<u32>,
    pub results: Vec<A>,
    pub errors: Option<Vec<String>>,
}

pub trait Article {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct EmailedArticle {
    pub r#abstract: String,
    pub adx_keywords: Option<String>,
    pub asset_id: Option<u64>,
    pub byline: Option<String>,
    pub column: Option<String>,
    pub count_type: Option<String>,
    pub eta_id: Option<u64>,
    pub id: u64,
    pub media: Vec<Media>,
    pub nytdsection: Option<String>,
    pub published_date: String,
    pub section: String,
    pub share_count: Option<u32>,
    pub source: Option<String>,
    pub subsection: Option<String>,
    pub title: String,
    pub r#type: String,
    pub updated: Option<String>,
    pub uri: String,
    pub url: String,
}
impl Article for EmailedArticle {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct ViewedArticle {
    pub r#abstract: String,
    pub adx_keywords: Option<String>,
    pub asset_id: Option<u64>,
    pub byline: Option<String>,
    pub column: Option<String>,
    pub id: Option<u64>,
    pub media: Vec<Media>,
    pub published_date: String,
    pub section: String,
    pub source: Option<String>,
    pub title: String,
    pub r#type: String,
    pub uri: String,
    pub url: String,
    pub views: Option<u32>,
}
impl Article for ViewedArticle {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct SharedArticle {
    pub r#abstract: String,
    pub adx_keywords: Option<String>,
    pub asset_id: Option<u64>,
    pub byline: Option<String>,
    pub column: Option<String>,
    pub count_type: Option<String>,
    pub eta_id: Option<u64>,
    pub id: u64,
    pub media: Vec<Media>,
    pub nytdsection: String,
    pub published_date: String,
    pub section: String,
    pub share_count: Option<u32>,
    pub source: String,
    pub subsection: Option<String>,
    pub title: String,
    pub r#type: String,
    pub updated: Option<String>,
    pub uri: String,
    pub url: String,
}
impl Article for SharedArticle {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct Media {
    pub r#type: String,
    pub subtype: Option<String>,
    pub caption: Option<String>,
    pub copyright: Option<String>,
    pub approved_for_syndication: u8,

    #[serde(rename = "media-metadata")]
    pub media_metadata: Vec<MediaMetadata>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct MediaMetadata {
    pub url: String,
    pub format: String,
    pub height: u32,
    pub width: u32,
}
