use std::convert::Infallible;

pub struct Lines<'a>(&'a str);

impl<'a> Lines<'a> {
    pub fn iter(&self) -> impl Iterator<Item = &'a str> {
        self.0.lines()
    }
}

impl<'a> TryFrom<&'a str> for Lines<'a> {
    type Error = Infallible;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}
