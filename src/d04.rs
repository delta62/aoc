use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Field {
    Byr(bool),
    Iyr(bool),
    Eyr(bool),
    Hgt(bool),
    Hcl(bool),
    Ecl(bool),
    Pid(bool),
    Cid,
}

use Field::*;

struct ParseError;

impl FromStr for Field {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref HEIGHT: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
            static ref HAIR: Regex = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
            static ref EYE: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
            static ref PID: Regex = Regex::new(r"^\d{9}$").unwrap();
        }

        let mut x = input.split(':');
        let tag = x.next().unwrap();
        let val = x.next().unwrap();

        match tag {
            "byr" => {
                let valid = val.parse::<usize>().map(|x| x >= 1920 && x <= 2002);
                Ok(Field::Byr(valid.unwrap_or(false)))
            }
            "iyr" => {
                let valid = val.parse::<usize>().map(|x| x >= 2010 && x <= 2020);
                Ok(Field::Iyr(valid.unwrap_or(false)))
            }
            "eyr" => {
                let valid = val.parse::<usize>().map(|x| x >= 2020 && x <= 2030);
                Ok(Field::Eyr(valid.unwrap_or(false)))
            }
            "hgt" => {
                let valid = HEIGHT.captures(val)
                    .and_then(|captures| {
                        let val = captures[1].parse::<usize>().unwrap();
                        let unit = &captures[2];

                        match unit {
                            "cm" => Some(val >= 150 && val <= 193),
                            "in" => Some(val >= 59 && val <= 76),
                            _ => None,
                        }
                    })
                    .unwrap_or(false);
                Ok(Field::Hgt(valid))
            }
            "hcl" => Ok(Field::Hcl(HAIR.is_match(val))),
            "ecl" => Ok(Field::Ecl(EYE.is_match(val))),
            "pid" => Ok(Field::Pid(PID.is_match(val))),
            "cid" => Ok(Field::Cid),
            _ => Err(ParseError),
        }
    }
}

struct Passport {
    fields: HashSet<Field>,
}

impl Passport {
    pub fn new<I: IntoIterator<Item = Field>>(data: I) -> Self {
        let mut fields = HashSet::new();
        for field in data {
            fields.insert(field);
        }

        Self { fields }
    }

    fn has_required_fields(&self) -> bool {
        self.fields
            .iter()
            .filter(|x| Cid != **x)
            .count() > 6
    }

    fn is_valid(&self) -> bool {
        [ Byr(true), Iyr(true), Eyr(true), Hgt(true), Hcl(true), Ecl(true), Pid(true) ]
            .iter()
            .all(|field| self.fields.contains(field))
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Passport> {
    let mut passports = Vec::new();
    let mut tokens = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            passports.push(Passport::new(tokens.drain(..)));
        } else {
            line
                .split_whitespace()
                .filter_map(|x| Field::from_str(x).ok())
                .for_each(|t| tokens.push(t))
        }
    }

    if !tokens.is_empty() {
        passports.push(Passport::new(tokens.drain(..)));
    }

    passports
}

#[aoc(day4, part1)]
fn solve_part1(input: &[Passport]) -> usize {
    input
        .iter()
        .filter(|x| x.has_required_fields())
        .count()
}

#[aoc(day4, part2)]
fn solve_part2(input: &[Passport]) -> usize {
    input
        .iter()
        .filter(|x| x.is_valid())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        let input = parse(input);
        let result = solve_part1(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_invalid() {
        let input = r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        let input = parse(input);
        let result = solve_part2(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn part2_valid() {
        let input = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let input = parse(input);
        let result = solve_part2(&input);
        assert_eq!(result, 4);
    }

    #[test]
    fn byr_validation() {
        let valid = Field::from_str("byr:2002");
        let invalid = Field::from_str("byr:2003");

        assert_eq!(Ok(Byr(true)), valid);
        assert_eq!(Ok(Byr(false)), invalid);
    }

    #[test]
    fn hgt_validation() {
        let valid1 = Field::from_str("hgt:60in");
        let valid2 = Field::from_str("hgt:190cm");
        let invalid1 = Field::from_str("hgt:190in");
        let invalid2 = Field::from_str("hgt:190");

        assert_eq!(Ok(Hgt(true)), valid1);
        assert_eq!(Ok(Hgt(true)), valid2);
        assert_eq!(Ok(Hgt(false)), invalid1);
        assert_eq!(Ok(Hgt(false)), invalid2);
    }

    #[test]
    fn hcl_validation() {
        let valid = Field::from_str("hcl:#123abc");
        let invalid1 = Field::from_str("hcl:#123abz");
        let invalid2 = Field::from_str("hcl:123abc");

        assert_eq!(Ok(Hcl(true)), valid);
        assert_eq!(Ok(Hcl(false)), invalid1);
        assert_eq!(Ok(Hcl(false)), invalid2);
    }

    #[test]
    fn ecl_validation() {
        let valid = Field::from_str("ecl:brn");
        let invalid = Field::from_str("ecl:wat");

        assert_eq!(Ok(Ecl(true)), valid);
        assert_eq!(Ok(Ecl(false)), invalid);
    }

    #[test]
    fn pid_validation() {
        let valid = Field::from_str("pid:000000001");
        let invalid = Field::from_str("pid:0123456789");

        assert_eq!(Ok(Pid(true)), valid);
        assert_eq!(Ok(Pid(false)), invalid);
    }
}
