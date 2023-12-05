use reqwest::{blocking::Client, Result, StatusCode};
use std::{fs, io, path::Path};

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

    pub fn fetch(&self, year: u16, day: u8, out_dir: impl AsRef<Path>) -> Result<()> {
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");
        let session_token = &self.session_token;
        let response = self
            .client
            .get(url)
            .header("Cookie", format!("session={session_token}"))
            .send()?;

        if response.status() != StatusCode::OK {
            panic!("Failed to fetch");
        }

        self.save_to_file(&response.bytes()?, day, out_dir).unwrap();

        Ok(())
    }

    fn save_to_file(&self, bytes: &[u8], day: u8, out_dir: impl AsRef<Path>) -> io::Result<()> {
        let out_dir = out_dir.as_ref();
        fs::create_dir_all(out_dir)?;

        let out_file = out_dir.join(format!("day{day:02}.txt"));
        fs::write(out_file, bytes)
    }
}
