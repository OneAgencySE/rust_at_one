
extern crate proc_macro;

use crate::proc_macro::TokenStream;

use quote::quote;

use syn::{parse_macro_input, ItemStruct, Error};

fn check_fields(input: &ItemStruct, fields: Vec<&str>) -> Result<(), Error> {
    let has_required_fields = fields.iter().all(|f| 
        input.fields.iter().any(| field | match &field.ident {
            Some(name) if name == f => true,
            _ => false
        })
    );
    match has_required_fields {
        true => Ok(()),
        false => Err(Error::new(input.ident.span(), format!("the derive macro 'Dto' requires struct to contain fields: {:?}", vec!["id"])))
    }
}

#[proc_macro_derive(Dto)]
pub fn dto_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    // get the name of the type we want to implement the trait for
    let name = &input.ident;
    match check_fields(&input, vec!["id"]) {
        Ok(_) => {
            let expanded = quote! {
                impl crate::services::Dto for #name {
                    fn set_id(&mut self, id: String) {
                        self.id = Some(id)
                    }
                }
            };
            TokenStream::from(expanded)
        },
        Err(e) => e.to_compile_error().into()
    }

}

#[proc_macro_derive(Query)]
pub fn query_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    // get the name of the type we want to implement the trait for
    let name = &input.ident;

    match check_fields(&input, vec!["id", "name", "author"]) {
        Ok(_) => {
            let expanded = quote! {
                impl crate::services::Query for #name {
                    fn from_string_id(id: String) -> Self {
                        Post {
                            id: Some(id),
                            name: None,
                            author: None,
                        }
                    }
                }
            };
            TokenStream::from(expanded)
        },
        Err(e) => e.to_compile_error().into()
    }
}
