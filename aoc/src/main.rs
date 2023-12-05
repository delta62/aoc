#[cfg(test)]
#[macro_use]
mod test_helpers;

mod day01;
mod day02;
mod day03;
mod day04;
mod input;

use aoc_runner::Runner;
use std::env;

pub fn main() {
    dotenv::dotenv().unwrap();

    let session_token = env::var("AOC_SESSION").unwrap();

    Runner::new()
        .auto_download_with_token(session_token)
        .run_latest_day();
}
