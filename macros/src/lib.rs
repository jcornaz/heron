use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Layer)]
pub fn derive_layer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_ident = input.ident;

    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => panic!("Only enums can automatically derive Layer"),
    };

    assert!(variants.len() <= 16, "Reached the maximum of 16 layers");

    let from_bits = variants.iter().enumerate().map(|(index, variant)| {
        let bits: u16 = 1 << index;
        assert!(
            variant.fields.is_empty(),
            "Can only derive Layer for enums without fields"
        );
        let ident = &variant.ident;
        quote! { #bits => #enum_ident::#ident, }
    });

    let to_bits = variants.iter().enumerate().map(|(index, variant)| {
        let bits: u16 = 1 << index;
        assert!(
            variant.fields.is_empty(),
            "Can only derive Layer for enums without fields"
        );
        let ident = &variant.ident;
        quote! { #enum_ident::#ident => #bits, }
    });

    let expanded = quote! {
        impl heron_core::Layer for #enum_ident {
            fn from_bits(bits: u16) -> Self {
                match bits {
                    #(#from_bits)*
                    _ => panic!("No layer with this bits: {}", bits),
                }
            }
            fn to_bits(&self) -> u16 {
                match self {
                    #(#to_bits)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
