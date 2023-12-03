use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    FnArg, Ident, ItemFn, LitInt, Result, ReturnType, Token, Type,
};

mod kw {
    syn::custom_keyword!(year);
    syn::custom_keyword!(day);
    syn::custom_keyword!(part);
}

enum Arg {
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

struct SolutionFunction {
    ident: Ident,
    input_type: Box<Type>,
    output_type: Box<Type>,
}

impl Parse for SolutionFunction {
    fn parse(input: ParseStream) -> Result<Self> {
        let f: ItemFn = input.parse()?;

        let ident = f.sig.ident;

        let output_type = match f.sig.output {
            ReturnType::Type(_, t) => t,
            _ => todo!(),
        };

        let input_arg = f.sig.inputs.into_iter().next().unwrap();
        let input_type = match input_arg {
            FnArg::Typed(t) => match *t.ty {
                Type::Reference(r) => r.elem,
                _ => t.ty,
            },
            _ => todo!(),
        };

        Ok(Self {
            ident,
            input_type,
            output_type,
        })
    }
}

struct PuzzleAnswer {
    year: u16,
    day: u8,
    part: u8,
}

impl Parse for PuzzleAnswer {
    fn parse(input: ParseStream) -> Result<Self> {
        let args = Punctuated::<Arg, Token![,]>::parse_terminated(input)?;

        let mut day = None;
        let mut part = None;
        let mut year = None;

        for arg in args {
            match arg {
                Arg::Day { value, .. } => day = Some(value.base10_parse()?),
                Arg::Year { value, .. } => year = Some(value.base10_parse()?),
                Arg::Part { value, .. } => part = Some(value.base10_parse()?),
            }
        }

        Ok(Self {
            year: year.unwrap(),
            day: day.unwrap(),
            part: part.unwrap_or(1),
        })
    }
}

#[proc_macro_attribute]
pub fn aoc(attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let answer = parse_macro_input!(attr as PuzzleAnswer);

    let f_item = item.clone();
    let f = parse_macro_input!(f_item as SolutionFunction);

    let struct_name = format_ident!("Day{}Part{}Solution", answer.day, answer.part);

    let solution_struct = quote! {
        #[derive(Default)]
        pub struct #struct_name;
    };

    let day = answer.day;
    let part = answer.part;
    let fn_name = f.ident;
    let input = f.input_type;
    let output = f.output_type;

    let solution_impl = quote! {
        impl crate::solution::PuzzleSolution for #struct_name {
            type Input<'a> = #input<'a>;
            type Output = #output;

            fn day(&self) -> u8 {
                #day
            }

            fn part(&self) -> u8 {
                #part
            }

            fn solve(&self, input: &Self::Input<'_>) -> Self::Output {
                #fn_name(&input)
            }
        }
    };

    let expanded = quote! {
        #solution_struct
        #solution_impl
    };

    item.extend(TokenStream::from(expanded));

    item
}
