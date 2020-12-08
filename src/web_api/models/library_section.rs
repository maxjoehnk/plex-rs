use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct LibrarySection {
    pub size: i64,
    #[serde(flatten)]
    pub paging: Option<Paging>,
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
    pub media_tag_version: u64,
    pub nocache: Option<bool>,
    pub thumb: Option<String>,
    #[serde(rename = "title1")]
    pub title: String,
    #[serde(rename = "title2")]
    pub secondary: Option<String>,
    pub view_group: Option<ViewGroup>,
    pub view_mode: Option<u64>,
    #[serde(default)]
    pub mixed_parents: bool,
    #[serde(rename = "Directory", default)]
    pub directory: Vec<Directory>,
    #[serde(rename = "Metadata", default)]
    pub metadata: Vec<Metadatum>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Paging {
    total_size: u64,
    offset: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Directory {
    Search(SearchDirectory),
    Folder(FolderDirectory),
    Section(SectionDirectory),
    Genre(GenreDirectory),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct SearchDirectory {
    pub prompt: String,
    pub search: bool,
    pub key: String,
    pub title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct FolderDirectory {
    pub key: String,
    pub title: String,
    #[serde(default)]
    pub secondary: bool,
    pub size: Option<u64>,
}

/// Genre or Decade
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GenreDirectory {
    pub fast_key: String,
    pub key: String,
    pub title: String,
    #[serde(rename = "type")]
    pub genre_type: Option<String>,
    pub thumb: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct SectionDirectory {
    pub allow_sync: bool,
    pub art: String,
    pub composite: String,
    pub filters: bool,
    pub refreshing: bool,
    pub thumb: String,
    pub key: String,
    #[serde(rename = "type")]
    pub directory_type: DirectoryType,
    pub title: String,
    pub agent: String,
    pub scanner: String,
    pub language: String,
    pub uuid: String,
    pub updated_at: Option<u64>,
    pub created_at: u64,
    pub content: bool,
    pub directory: bool,
    pub content_changed_at: u64,
    pub hidden: i64,
    #[serde(rename = "Location")]
    pub location: Vec<Location>,
    pub scanned_at: Option<u64>,
}

impl Directory {
    pub fn key(&self) -> String {
        match self {
            Directory::Search(dir) => dir.key.clone(),
            Directory::Folder(dir) => dir.key.clone(),
            Directory::Section(dir) => dir.key.clone(),
            Directory::Genre(dir) => dir.key.clone()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DirectoryType {
    Movie,
    Artist,
    Show,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
// TODO: folder view fails because it has no type field
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Metadatum {
    Artist(ArtistMetadatum),
    Album(AlbumMetadatum),
    Track(TrackMetadatum),
    Episode(EpisodeMetadatum),
    Movie(MovieMetadatum),
    Show(ShowMetadatum),
    Folder
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct ArtistMetadatum {
    pub rating_key: String,
    pub key: String,
    pub guid: String,
    pub title: String,
    pub summary: String,
    pub index: u64,
    pub thumb: Option<String>,
    pub art: Option<String>,
    pub added_at: u64,
    pub updated_at: Option<u64>,
    pub last_viewed_at: Option<u64>,
    #[serde(rename = "Genre", default)]
    pub genre: Vec<Tag>,
    #[serde(rename = "Country", default)]
    pub country: Vec<Tag>,
    pub title_sort: Option<String>,
    #[serde(default)]
    pub view_count: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct AlbumMetadatum {
    #[serde(default)]
    pub allow_sync: bool,
    #[serde(rename = "librarySectionID")]
    pub library_section_id: Option<i64>,
    pub library_section_title: Option<String>,
    #[serde(rename = "librarySectionUUID")]
    pub library_section_uuid: Option<String>,
    pub rating_key: String,
    pub key: String,
    pub parent_key: String,
    pub parent_rating_key: String,
    pub guid: String,
    pub parent_guid: String,
    pub title: String,
    pub parent_title: String,
    pub summary: String,
    pub index: u64,
    pub thumb: Option<String>,
    pub parent_thumb: Option<String>,
    pub added_at: u64,
    pub updated_at: Option<u64>,
    pub last_viewed_at: Option<u64>,
    pub originally_available_at: Option<String>,
    #[serde(rename = "Genre", default)]
    pub genre: Vec<Tag>,
    #[serde(rename = "Director", default)]
    pub director: Vec<Tag>,
    #[serde(rename = "Collection", default)]
    pub collections: Vec<Tag>,
    pub title_sort: Option<String>,
    pub art: Option<String>,
    #[serde(default)]
    pub view_count: u64,
    pub studio: Option<String>,
    pub year: Option<u64>,
    pub leaf_count: Option<u64>,
    /// Probably only available from recentlyAdded section
    pub viewed_leaf_count: Option<u64>,
    pub loudness_analysis_version: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct TrackMetadatum {
    pub rating_key: String,
    pub key: String,
    pub parent_rating_key: String,
    pub grandparent_rating_key: String,
    pub guid: String,
    pub parent_guid: String,
    pub grandparent_guid: String,
    pub title: String,
    pub title_sort: Option<String>,
    pub grandparent_key: String,
    pub parent_key: String,
    pub grandparent_title: String,
    pub parent_title: String,
    pub original_title: Option<String>,
    pub summary: String,
    pub index: Option<u64>,
    pub parent_index: u64,
    pub thumb: Option<String>,
    pub art: Option<String>,
    pub parent_thumb: Option<String>,
    pub parent_art: Option<String>,
    pub grandparent_thumb: Option<String>,
    pub grandparent_art: Option<String>,
    pub duration: Option<u64>,
    pub added_at: u64,
    pub updated_at: u64,
    #[serde(default)]
    pub view_count: u64,
    pub last_viewed_at: Option<u64>,
    #[serde(rename = "Media", default)]
    pub media: Vec<Media>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MovieMetadatum {
    pub rating_key: String,
    pub key: String,
    pub guid: String,
    pub studio: Option<String>,
    pub title: String,
    pub title_sort: Option<String>,
    pub original_title: Option<String>,
    pub content_rating: Option<String>,
    pub summary: String,
    pub rating: Option<f64>,
    pub audience_rating: Option<f64>,
    pub year: u64,
    pub tagline: Option<String>,
    pub thumb: Option<String>,
    pub art: Option<String>,
    pub duration: Option<u64>,
    pub originally_available_at: Option<String>,
    pub added_at: u64,
    pub updated_at: Option<u64>,
    pub audience_rating_image: Option<String>,
    pub primary_extra_key: Option<String>,
    pub rating_image: Option<String>,
    pub chapter_source: Option<String>,
    #[serde(rename = "Media", default)]
    pub media: Vec<Media>,
    #[serde(rename = "Genre", default)]
    pub genres: Vec<Tag>,
    #[serde(rename = "Director", default)]
    pub directors: Vec<Tag>,
    #[serde(rename = "Writer", default)]
    pub writers: Vec<Tag>,
    #[serde(rename = "Country", default)]
    pub countries: Vec<Tag>,
    #[serde(rename = "Collection", default)]
    pub collections: Vec<Tag>,
    #[serde(rename = "Role", default)]
    pub roles: Vec<Tag>,
    #[serde(default)]
    pub view_count: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct ShowMetadatum {
    pub rating_key: String,
    pub key: String,
    pub guid: String,
    pub studio: Option<String>,
    pub title: String,
    pub title_sort: Option<String>,
    pub original_title: Option<String>,
    pub content_rating: Option<String>,
    pub summary: String,
    pub index: u64,
    pub rating: Option<f64>,
    pub audience_rating: Option<f64>,
    pub year: Option<u64>,
    pub thumb: Option<String>,
    pub art: Option<String>,
    pub banner: Option<String>,
    pub theme: Option<String>,
    pub duration: Option<u64>,
    pub originally_available_at: Option<String>,
    pub leaf_count: u64,
    pub viewed_leaf_count: u64,
    pub child_count: u64,
    pub added_at: u64,
    pub updated_at: Option<u64>,
    pub audience_rating_image: Option<String>,
    pub primary_extra_key: Option<String>,
    pub rating_image: Option<String>,
    #[serde(rename = "Genre", default)]
    pub genres: Vec<Tag>,
    #[serde(rename = "Role", default)]
    pub roles: Vec<Tag>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct EpisodeMetadatum {
    pub rating_key: String,
    pub key: String,
    pub parent_rating_key: String,
    pub grandparent_rating_key: String,
    pub guid: String,
    pub parent_guid: String,
    pub grandparent_guid: String,
    pub title: String,
    pub title_sort: Option<String>,
    pub grandparent_key: String,
    pub parent_key: String,
    pub grandparent_title: String,
    pub parent_title: String,
    pub original_title: Option<String>,
    pub content_rating: Option<String>,
    pub summary: String,
    pub index: Option<u64>,
    pub parent_index: u64,
    pub thumb: Option<String>,
    pub art: Option<String>,
    pub parent_thumb: Option<String>,
    pub parent_art: Option<String>,
    pub grandparent_thumb: Option<String>,
    pub grandparent_art: Option<String>,
    pub grandparent_theme: Option<String>,
    pub duration: Option<u64>,
    pub rating: Option<f64>,
    pub year: Option<u64>,
    pub originally_available_at: Option<String>,
    pub added_at: u64,
    pub updated_at: u64,
    pub chapter_source: Option<String>,
    #[serde(default)]
    pub view_count: u64,
    pub last_viewed_at: Option<u64>,
    #[serde(rename = "Media", default)]
    pub media: Vec<Media>,
    #[serde(rename = "Writer", default)]
    pub writers: Vec<Tag>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Tag {
    pub tag: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Location {
    pub id: i64,
    pub path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ViewGroup {
    Album,
    Secondary,
    Artist,
    Movie,
    Show,
    Track,
    Episode,
    Albums,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub id: u64,
    pub duration: Option<u64>,
    pub bitrate: Option<u64>,
    pub container: Option<String>,
    pub has_64bit_offsets: Option<bool>,
    #[serde(flatten)]
    pub audio: Option<AudioMedia>,
    #[serde(flatten)]
    pub video: Option<VideoMedia>,
    #[serde(rename = "Part")]
    pub parts: Vec<Part>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct AudioMedia {
    pub audio_channels: u64,
    pub audio_codec: String,
    pub audio_profile: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct VideoMedia {
    pub width: u64,
    pub height: u64,
    pub aspect_ratio: Option<f64>,
    pub video_codec: String,
    pub video_resolution: String,
    pub video_frame_rate: String,
    pub video_profile: String,
    #[serde(default)]
    pub optimized_for_streaming: u64,
    pub display_offset: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Part {
    pub id: u64,
    pub key: String,
    pub duration: Option<u64>,
    pub file: String,
    pub size: u64,
    pub container: Option<String>,
    pub audio_profile: Option<String>,
    pub video_profile: Option<String>,
    // TODO: map to bool
    pub has_thumbnail: Option<String>,
    #[serde(default)]
    pub has_chapter_video_stream: bool,
    pub has_64bit_offsets: Option<bool>,
    #[serde(default)]
    pub optimized_for_streaming: bool,
    pub packet_length: Option<u64>,
}
