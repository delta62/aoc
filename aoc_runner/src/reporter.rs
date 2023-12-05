use crate::error::Result;

#[derive(Debug)]
pub struct DayReport {
    pub year: u16,
    pub day: u8,
    pub part1: Option<Result<String>>,
    pub part2: Option<Result<String>>,
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

    pub fn report_part(part: usize, result: &Result<String>) {
        let result = result.as_ref().unwrap();
        println!("    Part {part}: {result}");
    }
}

impl Reporter for DefaultReporter {
    fn report_day(&self, report: &DayReport) {
        println!("Advent of Code {} - Day {}", report.year, report.day);
        report
            .part1
            .as_ref()
            .map(|result| Self::report_part(1, result));
        report
            .part2
            .as_ref()
            .map(|result| Self::report_part(2, result));
    }

    fn report_no_solutions(&self) {
        eprintln!("No solutions found");
    }
}
