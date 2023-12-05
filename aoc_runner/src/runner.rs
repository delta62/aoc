use crate::{
    downloader::Downloader,
    error::{RunnerError, RunnerResult},
    reporter::{DayReport, DefaultReporter, Reporter, RunResult},
};
use aoc_runner_core::{inventory, UniversalSolution};
use itertools::Itertools;
use std::path::{Path, PathBuf};

type Solution = &'static (dyn UniversalSolution + Sync);

pub struct Runner {
    downloader: Option<Downloader>,
    input_path: PathBuf,
    reporter: Box<dyn Reporter>,
    solutions: Vec<Box<Solution>>,
}

impl Runner {
    pub fn new() -> Self {
        let downloader = None;
        let reporter = Box::new(DefaultReporter::new());
        let input_path = Path::new("input").to_path_buf();

        let mut solutions: Vec<_> = inventory::iter::<Solution>
            .into_iter()
            .map(|solution| Box::new(*solution))
            .collect();

        solutions.sort_by(|a, b| {
            a.year()
                .cmp(&b.year())
                .then(a.day().cmp(&b.day()))
                .then(a.part().cmp(&b.part()))
        });

        Self {
            downloader,
            input_path,
            reporter,
            solutions,
        }
    }

    pub fn with_reporter<R>(mut self, reporter: R) -> Self
    where
        R: Reporter + 'static,
    {
        self.reporter = Box::new(reporter);
        self
    }

    pub fn with_input_path(mut self, path: impl AsRef<Path>) -> Self {
        self.input_path = path.as_ref().to_path_buf();
        self
    }

    pub fn auto_download_with_token(mut self, session_token: impl Into<String>) -> Self {
        self.downloader = Some(Downloader::new(session_token));
        self
    }

    fn input_path(&self, year: u16, day: u8) -> String {
        // SAFETY: This value came from a string, so it can be put back into one
        let base = self.input_path.to_str().unwrap();
        format!("{base}/{year}/day{day:02}.txt")
    }

    fn downloader_opt(&self, year: u16, day: u8) -> Option<&Downloader> {
        self.downloader.as_ref().and_then(|downloader| {
            let input_path = self.input_path(year, day);
            let already_have_input = Path::new(&input_path).exists();

            if !already_have_input {
                None
            } else {
                Some(downloader)
            }
        })
    }

    fn run_day(&self, year: u16, day: u8) -> RunnerResult<()> {
        if let Some(downloader) = self.downloader_opt(year, day) {
            // SAFETY: This value came from a string, so it can be put back into one
            let base = self.input_path.to_str().unwrap();
            let out_dir = format!("{base}/{year}");
            downloader
                .fetch(year, day, &out_dir)
                .map_err(RunnerError::DownloadError)?;
        }

        let input = std::fs::read(self.input_path(year, day)).map_err(RunnerError::IoError)?;
        let results = self
            .solutions
            .iter()
            .filter(|s| s.year() == year && s.day() == day)
            .map(|s| RunResult {
                part: s.part(),
                answer: s.solve(&input),
            })
            .collect();

        let report = DayReport { year, day, results };
        self.reporter.report_day(&report);

        Ok(())
    }

    pub fn run_latest_day(&self) -> RunnerResult<()> {
        if let Some(latest) = self.solutions.last() {
            self.run_day(latest.year(), latest.day())
        } else {
            self.reporter.report_no_solutions();
            Ok(())
        }
    }

    pub fn run_all(&self) -> RunnerResult<()> {
        if self.solutions.is_empty() {
            self.reporter.report_no_solutions();
            Ok(())
        } else {
            self.solutions
                .iter()
                .map(|s| (s.year(), s.day()))
                .unique()
                .map(|(year, day)| self.run_day(year, day))
                .try_collect()
        }
    }
}
