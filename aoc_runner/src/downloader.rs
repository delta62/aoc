use crate::error::{RunnerError, RunnerResult};
use reqwest::{blocking::Client, StatusCode};
use std::{fs, path::Path};

pub struct Downloader {
    client: Client,
    session_token: String,
}

impl Downloader {
    pub fn new(session_token: impl Into<String>) -> Self {
        let client = Client::new();
        let session_token = session_token.into();
        Self {
            client,
            session_token,
        }
    }

    pub fn fetch(&self, year: u16, day: u8, out_dir: impl AsRef<Path>) -> RunnerResult<()> {
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");
        let session_token = &self.session_token;
        let response = self
            .client
            .get(url)
            .header("Cookie", format!("session={session_token}"))
            .send()
            .map_err(RunnerError::DownloadError)?;

        match response.status() {
            StatusCode::OK => self.save_to_file(
                &response.bytes().map_err(RunnerError::DownloadError)?,
                day,
                out_dir,
            ),
            _status => {
                todo!("Received unexpected status code")
            }
        }
    }

    fn save_to_file(&self, bytes: &[u8], day: u8, out_dir: impl AsRef<Path>) -> RunnerResult<()> {
        let out_dir = out_dir.as_ref();
        fs::create_dir_all(out_dir).map_err(RunnerError::IoError)?;

        let out_file = out_dir.join(format!("day{day:02}.txt"));
        fs::write(out_file, bytes).map_err(RunnerError::IoError)
    }
}
