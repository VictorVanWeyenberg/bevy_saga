use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::DeriveInput;

#[derive(deluxe::ExtractAttributes, Debug)]
#[deluxe(attributes(response))]
struct DeriveBevyRequestExtractAttributes(Ident);

fn derive_bevy_request2(input: proc_macro2::TokenStream) -> deluxe::Result<proc_macro2::TokenStream> {
    let mut ast: DeriveInput = syn::parse2(input)?;
    let DeriveBevyRequestExtractAttributes(response) = deluxe::extract_attributes(&mut ast)?;
    let ident = &ast.ident;
    Ok(quote! {
        impl bevy_event_flow::Request for #ident {
            type Response = #response;
        }

        impl SystemInput for #ident {
            type Param<'i> = #ident;
            type Inner<'i> = #ident;

            fn wrap(this: Self::Inner<'_>) -> Self::Param<'_> {
                this
            }
        }
    })
}

#[proc_macro_derive(Request, attributes(response))]
pub fn derive_bevy_request(input: TokenStream) -> TokenStream {
    derive_bevy_request2(input.into()).unwrap().into()
}