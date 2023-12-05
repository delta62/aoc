use syn::{
    parse::{Parse, ParseStream},
    LitInt, Result, Token,
};

mod kw {
    syn::custom_keyword!(year);
    syn::custom_keyword!(day);
    syn::custom_keyword!(part);
}

pub(crate) enum Arg {
    Year(LitInt),
    Day(LitInt),
    Part(LitInt),
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::year) {
            input.parse::<kw::year>()?;
            input.parse::<Token![=]>()?;
            Ok(Self::Year(input.parse()?))
        } else if lookahead.peek(kw::day) {
            input.parse::<kw::day>()?;
            input.parse::<Token![=]>()?;
            Ok(Self::Day(input.parse()?))
        } else if lookahead.peek(kw::part) {
            input.parse::<kw::part>()?;
            input.parse::<Token![=]>()?;
            Ok(Self::Part(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}
