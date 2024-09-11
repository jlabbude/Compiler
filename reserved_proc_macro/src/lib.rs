extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Reserved, attributes(word))]
pub fn reserved_word_strings(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = input.ident;
    let mut match_display_arms = Vec::new();
    let mut match_try_from_str_arms = Vec::new();
    let mut match_try_from_string_arms = Vec::new();
    if let Data::Enum(enum_data) = input.data {
        enum_data.variants.iter().for_each(|variant| {
            let ident = &variant.ident;
            match &variant
                .attrs
                .iter()
                .find_map(|attr| {
                    if attr.path().is_ident("word") {
                        attr.parse_args::<syn::LitStr>().ok()
                    } else {
                        None
                    }
                })
                .map(|lit_str| lit_str.value())
            {
                Some(word) => {
                    match_display_arms.push(quote! {
                        #enum_name::#ident => #word,
                    });

                    match_try_from_str_arms.push(quote! {
                        #word => Ok(#enum_name::#ident),
                    });

                    match_try_from_string_arms.push(quote! {
                        #word => Ok(#enum_name::#ident),
                    })
                }
                None => {
                    panic!("All variants must have a #[word(\"...\")] attribute");
                }
            }
        });
    } else {
        panic!("#[derive(ReservedWordStrings)] is only applicable to enums");
    }
    let display_impl = quote! {
        impl std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let word = match self {
                    #(#match_display_arms)*
                };
                write!(f, "{}", word)
            }
        }
    };
    let try_from_str_impl = quote! {
        impl std::convert::TryFrom<&str> for #enum_name {
            type Error = String;

            fn try_from(word: &str) -> Result<Self, Self::Error> {
                match word {
                    #(#match_try_from_str_arms)*
                    _ => Err(word.to_string()),
                }
            }
        }
    };
    let try_from_string_impl = quote! {
        impl std::convert::TryFrom<String> for #enum_name {
            type Error = String;

            fn try_from(word: String) -> Result<Self, Self::Error> {
                match word.as_str() {
                    #(#match_try_from_string_arms)*
                    _ => Err(word),
                }
            }
        }
    };
    let expanded = quote! {
        #display_impl
        #try_from_str_impl
        #try_from_string_impl
    };
    TokenStream::from(expanded)
}
