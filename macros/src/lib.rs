use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(PhysicsLayer)]
pub fn derive_layer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_ident = input.ident;

    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => panic!("Only enums can automatically derive Layer"),
    };

    assert!(variants.len() <= 32, "Reached the maximum of 32 layers");

    let to_bits = variants.iter().enumerate().map(|(index, variant)| {
        let bits: u32 = 1 << index;
        assert!(
            variant.fields.is_empty(),
            "Can only derive Layer for enums without fields"
        );
        let ident = &variant.ident;
        quote! { #enum_ident::#ident => #bits, }
    });

    let all_bits: u32 = if variants.len() == 32 {
        0xffffffff
    } else {
        (1 << variants.len()) - 1
    };

    let expanded = quote! {
        impl heron::PhysicsLayer for #enum_ident {
            fn all_bits() -> u32 {
                #all_bits
            }

            fn to_bits(&self) -> u32 {
                match self {
                    #(#to_bits)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
