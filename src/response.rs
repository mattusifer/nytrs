use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct Response {
    status: String,
    copyright: String,
    num_results: u32,
    results: Vec<Result>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
struct Result {
    url: String,
    adx_keywords: String,
    subsection: String,
    share_count: Option<u32>,
    count_type: String,
    column: Option<String>,
    eta_id: u32,
    section: String,
    id: u64,
    asset_id: u64,
    nytdsection: String,
    byline: String,
    r#type: String,
    title: String,
    r#abstract: String,
    published_date: String,
    source: String,
    updated: String,
    media: Vec<Media>,
    uri: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
struct Media {
    r#type: String,
    subtype: String,
    caption: String,
    copyright: String,
    approved_for_syndication: u8,

    #[serde(rename = "media-metadata")]
    media_metadata: Vec<MediaMetadata>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
struct MediaMetadata {
    url: String,
    format: String,
    height: u32,
    width: u32,
}
