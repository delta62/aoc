use crate::{error::Result, solution::PuzzleAnswer};

#[derive(Debug)]
pub struct RunResult {
    pub part: u8,
    pub answer: Result<PuzzleAnswer>,
}

#[derive(Debug)]
pub struct DayReport {
    pub year: u16,
    pub day: u8,
    pub results: Vec<RunResult>,
}

pub trait Reporter {
    /// Show the user results of running their solution for a given day
    fn report_day(&self, report: &DayReport);

    /// Let the user know that there are no solutions which can be run
    fn report_no_solutions(&self);
}

pub struct DefaultReporter;

impl DefaultReporter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn report_part(result: &RunResult) {
        let part = result.part;

        match &result.answer {
            Ok(res) => {
                let parse_duration = res.parse_duration;
                let solve_duration = res.solve_duration;
                let answer = &res.result;

                println!("    part {part}: {answer}");
                println!("      parse: {parse_duration:?}");
                println!("      solve: {solve_duration:?}");
                println!();
            }
            Err(err) => {
                println!("    part {part}: error");
                println!("      {err}");
            }
        }
    }
}

impl Reporter for DefaultReporter {
    fn report_day(&self, report: &DayReport) {
        println!("Advent of Code {} - Day {}", report.year, report.day);

        for result in &report.results {
            Self::report_part(result);
        }
    }

    fn report_no_solutions(&self) {
        eprintln!("No solutions found");
    }
}
