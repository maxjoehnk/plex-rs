use serde::{Serialize, Deserialize};
use crate::web_api::models::Directory;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
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
