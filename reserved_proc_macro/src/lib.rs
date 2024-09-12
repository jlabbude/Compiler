extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
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
            variant.attrs.iter().for_each(|attr| {
                if attr.path().is_ident("word") {
                    let words = match attr.parse_args::<syn::LitStr>() {
                        Ok(lit) => vec![lit.value()],
                        Err(_) => attr
                            .parse_args::<syn::ExprArray>()
                            .unwrap()
                            .elems
                            .iter()
                            .map(|expr| expr.to_token_stream().to_string().replace("\"", ""))
                            .collect::<Vec<String>>(), // sketchy
                    };
                    words.iter().for_each(|word| {
                        match_try_from_str_arms.push(quote! {
                            #word => Ok(#enum_name::#ident),
                        });

                        match_try_from_string_arms.push(quote! {
                            #word => Ok(#enum_name::#ident),
                        });
                    });
                    let word = &words[0];
                    match_display_arms.push(quote! {
                        #enum_name::#ident => #word,
                    });
                } else {
                    panic!("All variants must have a #[word(\"...\")] attribute");
                }
            });
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
