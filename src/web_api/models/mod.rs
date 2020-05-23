use serde::{Deserialize, Serialize};

pub use library_section::*;
pub use library_sections::*;
pub use search_results::*;
pub use server_information::*;

pub(crate) mod library_section;
pub(crate) mod library_sections;
pub(crate) mod search_results;
pub(crate) mod server_information;

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaContainer<T> {
    #[serde(rename = "MediaContainer")]
    pub media_container: T,
}

