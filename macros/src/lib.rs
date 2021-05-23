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

    let to_bits = variants.iter().enumerate().map(|(index, variant)| {
        let bits: u16 = 1 << index;
        assert!(
            variant.fields.is_empty(),
            "Can only derive Layer for enums without fields"
        );
        let ident = &variant.ident;
        quote! { #enum_ident::#ident => #bits, }
    });

    let mut all: u16 = 1;

    for _ in 1..variants.len() {
        all <<= 1;
        all += 1;
    }

    let expanded = quote! {
        impl heron::Layer for #enum_ident {
            fn all_bits() -> u16 {
                #all
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
