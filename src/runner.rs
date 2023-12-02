use num_traits::PrimInt;

pub trait Day<'a> {
    type Input: TryFrom<&'a str>;
    type Result: PrimInt + Default;

    fn year() -> usize;

    fn day() -> usize;

    fn part1(&mut self, input: Self::Input) -> Self::Result;

    fn part2(&mut self, input: Self::Input) -> Self::Result {
        let _ = input;
        Default::default()
    }
}

#[derive(Debug)]
pub enum Error<E: std::fmt::Debug> {
    ParseError(E),
    RunError(String),
}

pub type DayResult<T, E> = Result<T, Error<E>>;

pub fn run_part1<'a, D>(
    input: &'a str,
) -> DayResult<D::Result, <D::Input as TryFrom<&'a str>>::Error>
where
    D: Day<'a> + Default,
    <D::Input as TryFrom<&'a str>>::Error: std::fmt::Debug,
{
    let mut day = D::default();
    let input = input.try_into().map_err(Error::ParseError)?;
    let result = day.part1(input);

    Ok(result)
}

pub fn run_part2<'a, D>(input: &'a str) -> Result<D::Result, <D::Input as TryFrom<&'a str>>::Error>
where
    D: Day<'a> + Default,
{
    let mut day = D::default();
    let input = input.try_into()?;
    Ok(day.part2(input))
}
