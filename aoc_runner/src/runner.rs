use crate::UniversalSolution;

pub struct Runner {
    solutions: Vec<Box<&'static (dyn UniversalSolution + Sync)>>,
}

impl Runner {
    pub fn new() -> Self {
        let mut solutions = Vec::with_capacity(50);

        for s in inventory::iter::<&(dyn UniversalSolution + Sync + 'static)> {
            solutions.push(Box::new(*s));
        }

        solutions.sort_by(|a, b| {
            a.year()
                .cmp(&b.year())
                .then(a.day().cmp(&b.day()))
                .then(a.part().cmp(&b.part()))
        });

        Self { solutions }
    }

    fn run_solution(solution: &Box<&'static (dyn UniversalSolution + Sync)>) {
        let year = solution.year();
        let day = solution.day();
        let part = solution.part();
        let input_path = format!("aoc/input/{year}/day{day:02}.txt");
        let input = std::fs::read(input_path).unwrap();

        match solution.solve(&input) {
            Ok(answer) => println!("d{day}p{part} [OK]: {answer}"),
            Err(err) => eprintln!("d{day}p{part} [ERROR]: {err}"),
        }
    }

    pub fn run_latest_day(&self) {
        if let Some(latest) = self.solutions.last() {
            self.solutions
                .iter()
                .filter(|s| s.year() == latest.year() && s.day() == latest.day())
                .for_each(Self::run_solution);
        }
    }

    pub fn run_all(&self) {
        for s in &self.solutions {
            Self::run_solution(s);
        }
    }
}
