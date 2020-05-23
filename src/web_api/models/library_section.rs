use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct LibrarySection {
    pub size: i64,
    pub allow_sync: bool,
    pub art: Option<String>,
    pub content: Option<String>,
    pub identifier: String,
    #[serde(rename = "librarySectionID")]
    pub library_section_id: Option<i64>,
    pub library_section_title: Option<String>,
    #[serde(rename = "librarySectionUUID")]
    pub library_section_uuid: Option<String>,
    pub media_tag_prefix: String,
    pub media_tag_version: i64,
    pub nocache: Option<bool>,
    pub thumb: Option<String>,
    #[serde(rename = "title1")]
    pub title: String,
    #[serde(rename = "title2")]
    pub secondary: Option<String>,
    pub view_group: Option<String>,
    pub view_mode: i64,
    #[serde(rename = "Directory")]
    pub directory: Vec<Directory>,
    #[serde(rename = "Metadata", default)]
    pub metadata: Vec<Metadatum>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Directory {
    pub allow_sync: Option<bool>,
    pub art: Option<String>,
    pub key: String,
    pub title: String,
    pub secondary: Option<bool>,
    pub prompt: Option<String>,
    pub search: Option<bool>,
    pub composite: Option<String>,
    pub filters: Option<bool>,
    pub refreshing: Option<bool>,
    pub thumb: Option<String>,
    #[serde(rename = "type")]
    pub directory_type: Option<DirectoryType>,
    pub agent: Option<String>,
    pub scanner: Option<String>,
    pub language: Option<String>,
    pub uuid: Option<String>,
    pub updated_at: Option<i64>,
    pub created_at: Option<i64>,
    pub content: Option<bool>,
    #[serde(default)]
    pub directory: bool,
    pub content_changed_at: i64,
    pub hidden: i64,
    #[serde(rename = "Location", default)]
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
pub struct Metadatum {
    #[serde(rename = "ratingKey")]
    pub rating_key: String,
    pub key: String,
    pub guid: String,
    #[serde(rename = "type")]
    pub metadatum_type: ViewGroup,
    pub title: String,
    pub summary: String,
    pub index: i64,
    pub thumb: Option<String>,
    #[serde(rename = "addedAt")]
    pub added_at: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
    #[serde(rename = "Genre")]
    pub genre: Option<Vec<Country>>,
    #[serde(rename = "Country")]
    pub country: Option<Vec<Country>>,
    #[serde(rename = "titleSort")]
    pub title_sort: Option<String>,
    pub art: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Country {
    pub tag: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ViewGroup {
    #[serde(rename = "artist")]
    Artist,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Location {
    pub id: i64,
    pub path: String,
}
