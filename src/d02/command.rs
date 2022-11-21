use std::str::FromStr;

#[derive(Debug)]
pub enum MoveCommand {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for MoveCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MoveCommand::*;

        let mut words = s.split_ascii_whitespace();
        let direction = words.next();
        let magnitude = words.next().unwrap().parse().unwrap();

        match direction {
            Some("forward") => Ok(Forward(magnitude)),
            Some("up") => Ok(Up(magnitude)),
            Some("down") => Ok(Down(magnitude)),
            _ => Err(()),
        }
    }
}
