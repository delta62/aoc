#![feature(iterator_try_collect)]

#[cfg(test)]
#[macro_use]
mod test_helpers;
#[macro_use]
mod input;
mod args;
mod days;

use aoc_runner::{Runner, RunnerResult};
use args::Args;
use clap::Parser;
use std::env;

pub fn main() -> RunnerResult<()> {
    if dotenv::dotenv().is_err() {
        eprintln!("Warning: No .env file found");
    }

    let session_token = env::var("AOC_SESSION").expect("No session token is set");
    let args = Args::parse();

    let runner = Runner::new()
        .auto_download_with_token(session_token)
        .with_input_path("input");

    if args.all {
        runner.run_all()?;
    } else {
        runner.run_latest_day()?;
    }

    Ok(())
}
