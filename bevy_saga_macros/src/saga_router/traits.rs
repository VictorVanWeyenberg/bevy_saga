use crate::saga_router::{util, InputEnumMetaData, InputVariantMetaData};
use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use crate::saga_router::util::*;

pub fn generate_traits(input_enum: &InputEnumMetaData) -> Vec<TokenStream> {
    let mut traits = vec![];
    let mut generics_stack = vec![];
    let router_type = router_struct_name(input_enum);
    for window in input_enum.variants.iter().tuple_windows() {
        traits.push(trait_from_variants(window, &mut generics_stack))
    }
    traits.push(last_trait_from_variant(router_type, input_enum.variants.last().unwrap(), generics_stack));
    traits
}

fn trait_from_variants(
    (current, next): (&InputVariantMetaData, &InputVariantMetaData),
    generics_stack: &mut Vec<Ident>,
) -> TokenStream {
    let trait_name = trait_name(current);
    let trait_method_name = trait_method_name(current);
    let trait_parameter_type = handler_field_type(current);
    let trait_parameter_marker = to_marker_generic_type(current);
    let trait_parameter_name = trait_parameter_name(current);
    let constraint = to_generic_constraint(current);
    let return_trait = util::trait_name(next);
    let tokens = quote! {
        trait #trait_name<Source, MarkerSource, #(#generics_stack, )*> {
            fn #trait_method_name<#trait_parameter_type, #trait_parameter_marker>(
                self,
                #trait_parameter_name: #trait_parameter_type,
            ) -> impl #return_trait<Source, MarkerSource, #(#generics_stack, )* #trait_parameter_type>
            where
                #constraint;
        }
    };
    generics_stack.push(trait_parameter_type);
    tokens
}

fn last_trait_from_variant(router_type: Ident, variant: &InputVariantMetaData, generics_stack: Vec<Ident>) -> TokenStream {
    let trait_name = trait_name(variant);
    let trait_method_name = trait_method_name(variant);
    let trait_parameter_type = handler_field_type(variant);
    let trait_parameter_marker = to_marker_generic_type(variant);
    let trait_parameter_name = trait_parameter_name(variant);
    let constraint = to_generic_constraint(variant);
    quote! {
        trait #trait_name<Source, MarkerSource, #(#generics_stack, )*> {
            fn #trait_method_name<#trait_parameter_type, #trait_parameter_marker>(
                self,
                #trait_parameter_name: #trait_parameter_type,
            ) -> #router_type<Source, #(#generics_stack, )* #trait_parameter_type>
            where
                #constraint;
        }
    }
}
