use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(metadata))]
struct DeriveBevyRequestExtractAttributes {
    type_: String,
}

fn derive_bevy_request2(input: proc_macro2::TokenStream) -> deluxe::Result<proc_macro2::TokenStream> {
    let mut ast: DeriveInput = syn::parse2(input)?;
    let DeriveBevyRequestExtractAttributes { type_ } = deluxe::extract_attributes(&mut ast)?;
    let ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics BevyRequest for #ident #type_generics #where_clause {
            type BevyResponse = #type_
        }
    })
}

#[proc_macro_derive(BevyRequest, attributes(response))]
pub fn derive_bevy_request(input: TokenStream) -> TokenStream {
    derive_bevy_request2(input.into()).unwrap().into()
}