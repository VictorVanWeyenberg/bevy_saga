use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

fn derive_bevy_request2(input: proc_macro2::TokenStream) -> deluxe::Result<proc_macro2::TokenStream> {
    let ast: DeriveInput = syn::parse2(input)?;
    let ident = &ast.ident;
    Ok(quote! {
        impl bevy_saga::SagaEvent for #ident {
        }

        impl bevy::prelude::SystemInput for #ident {
            type Param<'i> = #ident;
            type Inner<'i> = #ident;

            fn wrap(this: Self::Inner<'_>) -> Self::Param<'_> {
                this
            }
        }
    })
}

#[proc_macro_derive(SagaEvent)]
pub fn derive_bevy_request(input: TokenStream) -> TokenStream {
    derive_bevy_request2(input.into()).unwrap().into()
}