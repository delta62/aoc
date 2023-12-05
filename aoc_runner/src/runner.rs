use std::path::Path;

use crate::{
    downloader::Downloader,
    reporter::{DayReport, DefaultReporter, Reporter},
    solution::UniversalSolution,
};
use itertools::Itertools;

type Solution = &'static (dyn UniversalSolution + Sync);

pub struct Runner {
    downloader: Option<Downloader>,
    reporter: Box<dyn Reporter>,
    solutions: Vec<Box<Solution>>,
}

impl Runner {
    pub fn new() -> Self {
        let downloader = None;
        let reporter = Box::new(DefaultReporter::new());

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

    pub fn auto_download_with_token(mut self, session_token: impl Into<String>) -> Self {
        self.downloader = Some(Downloader::new(session_token));
        self
    }

    fn input_path(year: u16, day: u8) -> String {
        format!("aoc/input/{year}/day{day:02}.txt")
    }

    fn should_fetch_input(&self, year: u16, day: u8) -> bool {
        let input_path = Self::input_path(year, day);
        let already_have_input = Path::new(&input_path).exists();

        self.downloader.is_some() && !already_have_input
    }

    fn run_day(&self, year: u16, day: u8) {
        if self.should_fetch_input(year, day) {
            let out_dir = format!("aoc/input/{year}");
            self.downloader.as_ref().unwrap().fetch(year, day, &out_dir);
        }

        let input = std::fs::read(Self::input_path(year, day)).unwrap();

        let mut part1 = None;
        let mut part2 = None;

        self.solutions
            .iter()
            .filter(|s| s.year() == year && s.day() == day)
            .for_each(|s| {
                let result = s.solve(&input);

                match s.part() {
                    1 => part1 = Some(result),
                    2 => part2 = Some(result),
                    _ => todo!(),
                }
            });

        let report = DayReport {
            year,
            day,
            part1,
            part2,
        };
        self.reporter.report_day(&report);
    }

    pub fn run_latest_day(&self) {
        if let Some(latest) = self.solutions.last() {
            self.run_day(latest.year(), latest.day())
        } else {
            self.reporter.report_no_solutions();
        }
    }

    pub fn run_all(&self) {
        if self.solutions.is_empty() {
            self.reporter.report_no_solutions();
        } else {
            self.solutions
                .iter()
                .map(|s| (s.year(), s.day()))
                .unique()
                .for_each(|(year, day)| self.run_day(year, day));
        }
    }
}
