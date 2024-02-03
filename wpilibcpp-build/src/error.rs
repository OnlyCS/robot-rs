use async_std::io;
use thiserror::Error;
use zip_extract::ZipExtractError;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("Error with surf: {0}")]
    Surf(surf::Error),

    #[error("IO: {0}")]
    IO(#[from] io::Error),

    #[error("Zip: {0}")]
    Zip(#[from] ZipExtractError),
}

impl From<surf::Error> for DownloadError {
    fn from(value: surf::Error) -> Self {
        Self::Surf(value)
    }
}
