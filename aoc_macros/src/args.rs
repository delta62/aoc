use syn::{
    parse::{Parse, ParseStream},
    LitInt, Result, Token
};

mod kw {
    syn::custom_keyword!(year);
    syn::custom_keyword!(day);
    syn::custom_keyword!(part);
}

pub(crate) enum Arg {
    Year {
        year_token: kw::year,
        eq_token: Token![=],
        value: LitInt,
    },
    Day {
        day_token: kw::day,
        eq_token: Token![=],
        value: LitInt,
    },
    Part {
        part_token: kw::part,
        eq_token: Token![=],
        value: LitInt,
    },
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::year) {
            Ok(Self::Year {
                year_token: input.parse::<kw::year>()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else if lookahead.peek(kw::day) {
            Ok(Self::Day {
                day_token: input.parse::<kw::day>()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else if lookahead.peek(kw::part) {
            Ok(Self::Part {
                part_token: input.parse::<kw::part>()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}
