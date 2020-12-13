use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Http Error {0}")]
    HttpError(#[from] reqwest::Error)
}
