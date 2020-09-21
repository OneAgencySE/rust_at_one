
extern crate proc_macro;

use crate::proc_macro::TokenStream;

use quote::quote;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Dto)]
pub fn dto_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // get the name of the type we want to implement the trait for
    let name = &input.ident;

    let expanded = quote! {
        impl crate::services::Dto for #name {
            fn set_id(&mut self, id: String) {
                self.id = Some(id)
            }
        }
    };

    TokenStream::from(expanded)
}
