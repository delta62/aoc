use super::MoveCommand;

#[derive(Default)]
pub struct Position {
    aim: i32,
    x: i32,
    depth: i32,
}

impl Position {
    pub fn update(self, cmd: &MoveCommand) -> Self {
        use MoveCommand::*;

        match cmd {
            Forward(x) => Self {
                x: self.x + x,
                ..self
            },
            Up(y) => Self {
                depth: self.depth - y,
                ..self
            },
            Down(y) => Self {
                depth: self.depth + y,
                ..self
            },
        }
    }

    pub fn update_aim(self, cmd: &MoveCommand) -> Self {
        use MoveCommand::*;

        dbg!(cmd, self.depth, self.aim, self.x);

        match cmd {
            Forward(x) => {
                let depth = self.depth - self.aim * x;
                let x = self.x + x;
                Self { x, depth, ..self }
            }
            Up(y) => Self {
                aim: self.aim + y,
                ..self
            },
            Down(y) => Self {
                aim: self.aim - y,
                ..self
            },
        }
    }

    pub fn as_solution(&self) -> i32 {
        self.x * self.depth
    }
}
