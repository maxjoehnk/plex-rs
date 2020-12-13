use reqwest::{Client, RequestBuilder};

use self::error::Error;
use self::models::*;

pub mod models;
pub mod error;

#[derive(Debug, Clone)]
pub struct PlexWebApi {
    base_url: String,
    auth_token: String,
    client: Client
}

impl PlexWebApi {
    pub fn new<S: Into<String>>(base_url: S, auth_token: S) -> Self {
        PlexWebApi {
            base_url: base_url.into(),
            auth_token: auth_token.into(),
            client: Client::new()
        }
    }

    pub async fn server_information(&self) -> Result<ServerInfo, Error> {
        let res: MediaContainer<_> = self.api_get(&format!("{}", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(res.media_container)
    }

    pub async fn library_sections(&self) -> Result<LibrarySections, Error> {
        let res: MediaContainer<_> = self.api_get(&format!("{}/library/sections", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(res.media_container)
    }

    pub async fn library_section<S: Into<String>>(&self, section: S) -> Result<LibrarySection, Error> {
        let res: MediaContainer<_> = self.api_get(&format!("{}/library/sections/{}", self.base_url, section.into()))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(res.media_container)
    }

    pub async fn search<Q: Into<String>>(&self, query: Q) -> Result<SearchResults, Error> {
        let res: MediaContainer<_> = self.api_get(&format!("{}/search", self.base_url))
            .query(&[("query", query.into())])
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(res.media_container)
    }

    fn api_get(&self, url: &str) -> RequestBuilder {
        self.client.get(url)
            .header("X-Plex-Token", &self.auth_token)
            .header("Accept", "application/json")
    }
}

#[cfg(test)]
mod test {
    use super::PlexWebApi;
    use crate::web_api::models::Directory;

    fn create_api() -> PlexWebApi {
        let url = env!("PLEX_BASE_URL");
        let token = env!("PLEX_TOKEN");

        PlexWebApi::new(url, token)
    }

    #[tokio::test]
    async fn test_server_information() {
        let api = create_api();

        let res = api.server_information().await;

        println!("{:?}", res);
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_library_sections() {
        let api = create_api();

        let res = api.library_sections().await;

        println!("{:?}", res);
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_directory_traversal() {
        let api = create_api();

        let sections = api.library_sections().await.unwrap();
        let mut errors = 0u64;
        let directories = sections.directories.into_iter().map(|dir| (String::new(), dir)).collect();
        let mut directories = iterate_directories(&api, &mut errors, directories).await;
        for _ in 0..2 {
            directories = iterate_directories(&api, &mut errors, directories).await;
        }

        assert_eq!(errors, 0);
    }

    async fn iterate_directories(api: &PlexWebApi, errors: &mut u64, directories: Vec<(String, Directory)>) -> Vec<(String, Directory)> {
        let mut next_directories = Vec::new();
        for (prefix, directory) in directories {
            let key = if prefix.is_empty() { directory.key() } else {
                format!("{}/{}", prefix, directory.key())
            };
            let res = api.library_section(&key).await;
            if let Ok(section) = res {
                let mut dirs = section.directory.clone().into_iter().map(|dir| (key.clone(), dir)).collect();
                next_directories.append(&mut dirs);
            }else {
                println!("{} => {:?}", &key, &res);
                *errors += 1;
            }
        }

        next_directories
    }
}
