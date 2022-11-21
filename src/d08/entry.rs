use std::{ops::BitOr, str::FromStr};

use crate::parse_helpers::ParseError;

pub struct Entry {
    signal_patterns: [SignalPattern; 10],
    output: [SignalPattern; 4],
}

impl Entry {
    pub fn output(&self) -> impl Iterator<Item = &'_ SignalPattern> {
        self.output.iter()
    }

    fn narrow<P>(&self, predicate: P) -> u8
    where
        P: Fn(u8) -> bool,
    {
        self.signal_patterns
            .iter()
            .find(|p| predicate(p.0))
            .unwrap()
            .0
    }

    pub fn output_val(&self) -> usize {
        let mut found = [0; 10];

        for patt in &self.signal_patterns {
            match patt.0.count_ones() {
                2 => found[1] = patt.0,
                3 => found[7] = patt.0,
                4 => found[4] = patt.0,
                7 => found[8] = patt.0,
                _ => {}
            }
        }

        found[9] =
            self.narrow(|p| p.count_ones() == 6 && ((found[4] | found[7]) ^ p).count_ones() == 1);

        found[3] = self.narrow(|p| {
            p.count_ones() == 5 && (found[9] ^ found[4] ^ found[1] ^ p).count_ones() == 1
        });

        found[2] = self.narrow(|p| p.count_ones() == 5 && (p ^ found[4]).count_ones() == 5);

        found[5] = self.narrow(|p| p != found[3] && p != found[2] && p.count_ones() == 5);

        found[0] = self.narrow(|p| {
            p != found[4] && p != found[1] && (found[8] ^ found[3] ^ found[1] ^ p).count_ones() == 2
        });

        found[6] = self.narrow(|p| p != found[0] && p != found[9] && p.count_ones() == 6);

        self.calculate_output(&found)
    }

    fn calculate_output(&self, found_digits: &[u8; 10]) -> usize {
        self.output.iter().fold(0, |acc, out| {
            let digit = (0..10)
                .find_map(|i| (found_digits[i] == out.0).then(|| i))
                .unwrap();

            acc * 10 + digit
        })
    }
}

impl FromStr for Entry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('|');

        let signal_patterns = parts
            .next()
            .ok_or(ParseError::UnexpectedEof)
            .map(|line| line.split_whitespace())
            .and_then(|pattern| {
                let mut bits = pattern.map(|p| p.parse());
                bits.try_collect::<Vec<_>>()
            })?;

        let signal_patterns =
            array_init::from_iter(signal_patterns).ok_or(ParseError::UnexpectedEof)?;

        let output = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|pattern| pattern.parse().unwrap());

        let output = array_init::from_iter(output).unwrap();

        Ok(Self {
            signal_patterns,
            output,
        })
    }
}

pub struct SignalPattern(u8);

impl SignalPattern {
    pub fn try_identify(&self) -> Option<u8> {
        match self.0.count_ones() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        }
    }
}

impl FromStr for SignalPattern {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits = s
            .chars()
            .map(Segment::try_from_char)
            .try_fold(0, |acc, n| n.map(|n| acc | n))?;

        Ok(Self(bits))
    }
}

impl BitOr<Segment> for u8 {
    type Output = u8;

    fn bitor(self, rhs: Segment) -> Self::Output {
        self | rhs.0
    }
}

struct Segment(u8);

impl Segment {
    fn try_from_char(c: char) -> Result<Self, ParseError> {
        match c {
            'a' => Ok(Self(0b0000_0001)),
            'b' => Ok(Self(0b0000_0010)),
            'c' => Ok(Self(0b0000_0100)),
            'd' => Ok(Self(0b0000_1000)),
            'e' => Ok(Self(0b0001_0000)),
            'f' => Ok(Self(0b0010_0000)),
            'g' => Ok(Self(0b0100_0000)),
            c => Err(ParseError::illegal_char(
                c,
                vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'],
            )),
        }
    }
}
