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

    pub fn fetch(&self, year: u16, day: u8, out_dir: impl AsRef<Path>) {
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");
        let response = self
            .client
            .get(url)
            .header("Cookie", format!("session={}", self.session_token))
            .send()
            .unwrap();

        if response.status() != StatusCode::OK {
            panic!("Failed to fetch");
        }

        let out_dir = out_dir.as_ref();
        fs::create_dir_all(out_dir).unwrap();

        let out_file = out_dir.join(format!("day{day:02}.txt"));
        fs::write(out_file, response.bytes().unwrap()).unwrap();
    }
}
