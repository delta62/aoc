use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    /// Run all solutions. By default, only the latest day's solutions are run.
    #[clap(short, long)]
    pub all: bool,
}
