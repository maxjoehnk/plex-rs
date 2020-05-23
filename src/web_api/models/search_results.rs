use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct SearchResults {
    pub size: i64,
    pub identifier: String,
    pub media_tag_prefix: String,
    pub media_tag_version: i64,
    #[serde(rename = "Metadata")]
    pub results: Vec<SearchResult>,
    #[serde(rename = "Provider")]
    pub provider: Vec<Provider>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct SearchResult {
    pub allow_sync: bool,
    #[serde(rename = "librarySectionID")]
    pub library_section_id: i64,
    pub library_section_title: String,
    #[serde(rename = "librarySectionUUID")]
    pub library_section_uuid: String,
    pub personal: bool,
    pub source_title: String,
    pub rating_key: String,
    pub key: String,
    pub parent_rating_key: String,
    pub guid: String,
    pub parent_guid: String,
    pub studio: String,
    #[serde(rename = "type")]
    pub metadatum_type: String,
    pub title: String,
    pub parent_key: String,
    pub parent_title: String,
    pub summary: String,
    pub index: i64,
    pub year: i64,
    pub thumb: String,
    pub art: String,
    pub parent_thumb: String,
    pub originally_available_at: String,
    pub added_at: i64,
    pub updated_at: i64,
    #[serde(rename = "Genre")]
    pub genre: Vec<Director>,
    #[serde(rename = "Director")]
    pub director: Vec<Director>,
    pub title_sort: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Director {
    pub tag: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Provider {
    pub key: String,
    pub title: String,
    #[serde(rename = "type")]
    pub provider_type: String,
}
