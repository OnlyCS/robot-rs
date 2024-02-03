#![feature(error_generic_member_access)]

extern crate async_std;
extern crate futures;
extern crate surf;
extern crate zip_extract;

pub mod error;
pub mod runner;

#[cfg(test)]
mod tests {
    use crate::runner::{wpilib, Runner};
    use async_std::task;
    use std::path::PathBuf;

    #[test]
    fn main() {
        task::block_on(
            Runner::new(wpilib::downloads("2024.2.1"), PathBuf::new()).download_and_unzip(),
        )
        .unwrap()
    }
}
