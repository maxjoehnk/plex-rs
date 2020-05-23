use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[cfg_attr(test, serde(deny_unknown_fields))]
pub struct LibrarySections {
    pub size: i64,
    pub allow_sync: bool,
    pub identifier: String,
    pub media_tag_prefix: String,
    pub media_tag_version: i64,
    pub title1: String,
    #[serde(rename = "Directory")]
    pub directories: Vec<Directory>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Directory {
    pub allow_sync: bool,
    pub art: String,
    pub composite: String,
    pub filters: bool,
    pub refreshing: bool,
    pub thumb: String,
    pub key: String,
    #[serde(rename = "type")]
    pub directory_type: String,
    pub title: String,
    pub agent: String,
    pub scanner: String,
    pub language: String,
    pub uuid: String,
    pub updated_at: i64,
    pub created_at: i64,
    pub content: bool,
    pub directory: bool,
    pub content_changed_at: i64,
    pub hidden: i64,
    #[serde(rename = "Location")]
    pub location: Vec<Location>,
    pub scanned_at: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DirectoryType {
    Movie,
    Artist,
    Show
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Location {
    pub id: i64,
    pub path: String,
}
