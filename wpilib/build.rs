#![feature(let_chains)]

extern crate async_std;
extern crate autocxx_build;
extern crate autocxx_engine;
extern crate thiserror;
extern crate wpilibcpp_build;

use std::{fs, path::PathBuf};
use thiserror::Error;
use wpilibcpp_build::{
    error::DownloadError,
    runner::{wpilib, Runner},
};

#[derive(Error, Debug)]
enum BuildError {
    #[error("Failed to download or extract WPILib: {0}")]
    Download(#[from] DownloadError),

    #[error("Failed to build autocxx: {0}")]
    Autocxx(#[from] autocxx_engine::BuilderError),
}

async fn main_async() -> Result<(), DownloadError> {
    let path = PathBuf::new().join("src").join("wpilib_cpp");

    if fs::metadata(&path).is_ok() {
        return Ok(());
    }

    Runner::new(wpilib::downloads("2024.2.1"), path)
        .download_and_unzip()
        .await?;

    Ok(())
}

fn main() -> Result<(), BuildError> {
    async_std::task::block_on(main_async())?;

    let include = [
        PathBuf::new().join("src"),
        PathBuf::new().join("src").join("wpilib_cpp"),
    ];

    autocxx_build::Builder::new("src/lib.rs", &include)
        .extra_clang_args(&["-std=c++20"])
        .build()?
        .flag_if_supported("-std=c++20")
        .compile("autocxx-wpilib");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/cpp.rs");

    Ok(())
}
