use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::parse_macro_input;

macro_rules! bail {
    ($span:expr, $($tt:tt)*) => {
        return Err(syn::Error::new_spanned($span, format!($($tt)*)))
    };
}

mod volatile;

#[proc_macro_derive(VolatileFieldAccess, attributes(access))]
pub fn derive_volatile(item: TokenStream) -> TokenStream {
    match volatile::derive_volatile(parse_macro_input!(item)) {
        Ok(items) => {
            let mut tokens = TokenStream2::new();
            for item in &items {
                item.to_tokens(&mut tokens);
            }
            tokens.into()
        }
        Err(e) => e.to_compile_error().into(),
    }
}
