mod args;
mod macros;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn aoc(attr: TokenStream, item: TokenStream) -> TokenStream {
    macros::aoc(attr, item)
}
