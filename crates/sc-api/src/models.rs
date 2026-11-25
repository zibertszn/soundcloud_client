use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchResponse {
    pub collection: Vec<Track>,
    pub next_href: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Track {
    pub id: u64,
    pub title: Option<String>,
    pub artwork_url: Option<String>,
    pub duration: u64,
    pub full_duration: Option<u64>,
    pub genre: Option<String>,
    pub likes_count: Option<u64>,
    pub permalink_url: Option<String>,
    pub media: Media,
    pub user: Option<User>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Media {
    pub transcodings: Vec<Transcoding>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transcoding {
    pub url: String,
    pub preset: String,
    pub duration: u64,
    pub snipped: bool,
    pub quality: String,
    pub format: StreamFormat,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StreamFormat {
    pub protocol: String,
    pub mime_type: String,
}
