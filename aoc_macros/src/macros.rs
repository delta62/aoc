use crate::args::Arg;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    FnArg, GenericParam, Generics, Ident, ImplItemType, ItemFn, Lifetime, LifetimeParam, PatType,
    Result, ReturnType, Token, Type,
};

struct SolutionFunction {
    ident: Ident,
    input_pat: PatType,
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
        let input_pat = match input_arg {
            FnArg::Typed(t) => t,
            _ => todo!(),
        };

        Ok(Self {
            ident,
            input_pat,
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
    let year = answer.year;
    let fn_name = f.ident;
    let output = f.output_type;

    let lifetime = LifetimeParam::new(Lifetime::new("'a", Span::call_site()));
    let lifetime = GenericParam::Lifetime(lifetime);
    let mut input_generics = Generics::default();
    input_generics.params.push(lifetime);

    let sometype = *f.input_pat.ty;

    // let foo = match sometype.clone() {
    //     Type::Array(_) => "array",
    //     Type::BareFn(_) => "barefn",
    //     Type::Group(_) => "group",
    //     Type::ImplTrait(_) => "impltrait",
    //     Type::Infer(_) => "infer",
    //     Type::Macro(_) => "macro",
    //     Type::Never(_) => "never",
    //     Type::Paren(_) => "paren",
    //     Type::Path(_) => "path",
    //     Type::Ptr(_) => "ptr",
    //     Type::Reference(_) => "reference",
    //     Type::Slice(_) => "slice",
    //     Type::TraitObject(_) => "traitobj",
    //     Type::Tuple(_) => "tuple",
    //     Type::Verbatim(_) => "verbatim",
    //     x => "wtf",
    // };

    // dbg!(foo);

    let input = ImplItemType {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        defaultness: None,
        type_token: Default::default(),
        ident: format_ident!("Input"),
        generics: input_generics,
        eq_token: Default::default(),
        ty: sometype,
        semi_token: Default::default(),
    };

    let solution_impl = quote! {
        impl aoc_runner::PuzzleSolution for #struct_name {
            #input
            type Output = #output;

            fn year(&self) -> u16 {
                #year
            }

            fn day(&self) -> u8 {
                #day
            }

            fn part(&self) -> u8 {
                #part
            }

            fn solve(&self, input: Self::Input<'_>) -> Self::Output {
                #fn_name(input)
            }
        }
    };

    let solution_collection_struct = quote! {
        ::aoc_runner::inventory::submit! {
            &#struct_name as &(dyn ::aoc_runner::UniversalSolution + Sync + 'static)
        }
    };

    let expanded = quote! {
        #solution_struct
        #solution_impl
        #solution_collection_struct
    };

    item.extend(TokenStream::from(expanded));

    item
}
