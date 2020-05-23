use serde::{Deserialize, Serialize};

pub use library_section::*;
pub use library_sections::*;
pub use search_results::*;
pub use server_information::*;

mod library_section;
mod library_sections;
mod search_results;
mod server_information;

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaContainer<T> {
    #[serde(rename = "MediaContainer")]
    pub media_container: T,
}

