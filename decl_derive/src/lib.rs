#![allow(warnings)]
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::Tokens;
use quote::ToTokens;

mod args;

#[proc_macro_derive(Declarative)]
pub fn derive_declarative(input: TokenStream) -> TokenStream {
    let input = syn::parse_derive_input(&input.to_string())
        .expect("failed to parse derive input");
    expand_derive_declarative(input).parse()
        .expect("failed to serialize rust")
}

fn expand_derive_declarative(input: syn::DeriveInput) -> Tokens {
    use syn::Body;
    use syn::VariantData;
    
    let ident = &input.ident;
    let generics = &input.generics;

    match input.body {
        Body::Struct(ref s) => derive_declarative_struct(ident, generics, s.fields()),
        Body::Enum(ref e) => unimplemented!(), 
    }
}

fn derive_declarative_struct(
    ident: &syn::Ident, 
    generics: &syn::Generics, 
    variant: &[syn::Field],
) -> Tokens 
{
    unimplemented!()
//     quote!(
//         impl #impl_generics Declarative<'buf> for #ty #ty_generics #where_clause {
//             type Argument = #argument;
//             
//             fn parse(buffer: &'buf [u8], argument: Self::Argument) -> DeclResult<'buf, Self> {
//                 // Size check?
//                 
//                 // For each field:
//                 //  - expand tags, and dropped in order
//                 //  - check for specialty annotations (length, relative_to)
//                 
//                 // Generate type.
//             }
//         }
//     )
}
