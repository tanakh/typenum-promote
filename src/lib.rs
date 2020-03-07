extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitInt};

fn gen_type(n: u64) -> proc_macro2::TokenStream {
    if n == 0 {
        quote! { typenum::UTerm }
    } else {
        let ty = gen_type(n / 2);
        let b = if n % 2 == 0 {
            quote! { typenum::B0 }
        } else {
            quote! { typenum::B1 }
        };
        quote! { typenum::UInt<#ty, #b> }
    }
}

/// Promote integer literal to type-level integer
///
/// ```
/// assert_eq!(<promote!(12345) as Unsigned>::to_u64(), 12345);
/// ```
///
#[proc_macro]
pub fn promote(item: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(item as LitInt);
    let value = lit.base10_parse::<u64>().unwrap();
    gen_type(value).into()
}
