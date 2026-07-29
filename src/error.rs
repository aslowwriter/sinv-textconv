
use std::{io, path::PathBuf};

use thiserror::Error;
#[derive(Error, Debug)]
pub enum SinvTextconvError {
    #[error("expected only one argument, got {0}")]
    IncorrectNumberArguments(usize),

    #[error("supplied path was not a file: {0}")]
    ArgNotAFile(String),

    #[error("Io error")]
    IoError(#[from] io::Error),
}
