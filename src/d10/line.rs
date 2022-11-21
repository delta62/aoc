use std::{collections::VecDeque, fmt::Display, str::FromStr};

#[derive(Clone, Copy)]
enum Char {
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftAngle,
    RightAngle,
    LeftCurly,
    RightCurly,
}

impl Char {
    fn closes(&self, other: Char) -> bool {
        match (self, other) {
            (Self::RightParen, Self::LeftParen) => true,
            (Self::RightBracket, Self::LeftBracket) => true,
            (Self::RightCurly, Self::LeftCurly) => true,
            (Self::RightAngle, Self::LeftAngle) => true,
            _ => false,
        }
    }

    fn score(&self) -> usize {
        match self {
            Self::RightParen => 3,
            Self::RightBracket => 57,
            Self::RightCurly => 1197,
            Self::RightAngle => 25137,
            _ => unreachable!(),
        }
    }

    fn autocomplete_score(&self) -> usize {
        match self {
            Self::RightParen => 1,
            Self::RightBracket => 2,
            Self::RightCurly => 3,
            Self::RightAngle => 4,
            _ => unreachable!(),
        }
    }

    fn is_closer(&self) -> bool {
        match self {
            Self::RightParen => true,
            Self::RightBracket => true,
            Self::RightCurly => true,
            Self::RightAngle => true,
            _ => false,
        }
    }

    fn closer(&self) -> Char {
        match self {
            Self::LeftParen => Char::RightParen,
            Self::LeftBracket => Char::RightBracket,
            Self::LeftCurly => Char::RightCurly,
            Self::LeftAngle => Char::RightAngle,
            _ => unreachable!(),
        }
    }
}

impl Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeftParen => write!(f, "("),
            Self::LeftBracket => write!(f, "["),
            Self::LeftCurly => write!(f, "{{"),
            Self::LeftAngle => write!(f, "<"),
            Self::RightParen => write!(f, ")"),
            Self::RightBracket => write!(f, "]"),
            Self::RightCurly => write!(f, "}}"),
            Self::RightAngle => write!(f, ">"),
        }
    }
}

impl TryFrom<char> for Char {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Self::LeftParen),
            ')' => Ok(Self::RightParen),
            '[' => Ok(Self::LeftBracket),
            ']' => Ok(Self::RightBracket),
            '{' => Ok(Self::LeftCurly),
            '}' => Ok(Self::RightCurly),
            '<' => Ok(Self::LeftAngle),
            '>' => Ok(Self::RightAngle),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
pub struct Line(VecDeque<Char>);

impl Line {
    pub fn score(&mut self) -> Option<usize> {
        let mut stack = Vec::with_capacity(self.0.len());

        while !self.0.is_empty() {
            let c = self.0.pop_front().unwrap();

            if !c.is_closer() {
                stack.push(c);
                continue;
            }

            let closes = stack.last().map(|&l| c.closes(l)).unwrap_or(false);
            if closes {
                stack.pop();
                continue;
            }

            return Some(c.score());
        }

        None
    }

    pub fn autocomplete_score(&mut self) -> usize {
        let mut stack = Vec::with_capacity(self.0.len());
        let mut sum = 0;

        while !self.0.is_empty() {
            let next = self.0.pop_front().unwrap();

            if next.is_closer() {
                let mut stack_next = stack.pop().unwrap();
                while !next.closes(stack_next) {
                    let generated_closer = stack_next.closer();
                    sum = sum * 5 + generated_closer.autocomplete_score();
                    stack_next = stack.pop().unwrap();
                }
            } else {
                stack.push(next);
            }
        }

        while !stack.is_empty() {
            let generated_closer = stack.pop().unwrap().closer();
            sum = sum * 5 + generated_closer.autocomplete_score();
        }

        sum
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec = s.chars().try_fold(VecDeque::new(), |mut acc, c| {
            acc.push_back(c.try_into()?);
            Ok(acc)
        });

        vec.map(|v| Self(v))
    }
}
