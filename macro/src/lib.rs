extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(TestDerive)]
pub fn debug_derive(_input: TokenStream) -> TokenStream {
    unimplemented!()
}

