use std::{error::Error, fmt::Display, io};

#[derive(Debug)]
pub enum RunnerError {
    IoError(io::Error),
    DownloadError(reqwest::Error),
}

impl Display for RunnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RunnerError::*;

        match self {
            IoError(err) => write!(f, "I/O error: {err}"),
            DownloadError(err) => write!(f, "Error downloading input: {err}"),
        }
    }
}

impl Error for RunnerError {}

pub type RunnerResult<T> = Result<T, RunnerError>;
