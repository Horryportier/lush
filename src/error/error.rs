use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LushError {
    #[error("Io error {0}")]
    IoErr(#[from] io::Error),
    #[error("Lush error {0}")]
    #[allow(dead_code)]
    LushErr(String),
}
