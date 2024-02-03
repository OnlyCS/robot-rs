use crate::error::*;
use std::{io::Cursor, path::PathBuf};

pub mod wpilib;

pub struct Runner {
    downloads: Vec<String>,
    out_dir: PathBuf,
}

impl Runner {
    pub fn new(downloads: Vec<String>, out_dir: PathBuf) -> Self {
        Self { downloads, out_dir }
    }

    pub async fn download_and_unzip(&self) -> Result<(), DownloadError> {
        let mut tasks = vec![];

        for i in &self.downloads {
            let out = self.out_dir.clone();

            let task = async move {
                let res = surf::get(&i).await?.body_bytes().await?;
                zip_extract::extract(Cursor::new(res), &out.join("raw"), false)?;

                Ok::<_, DownloadError>(())
            };

            tasks.push(task);
        }

        for res in futures::future::join_all(tasks).await {
            res?;
        }

        Ok(())
    }
}
