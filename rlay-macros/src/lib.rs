use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn t(item: TokenStream) -> TokenStream {
    r#"println!("Hello!")"#.parse().unwrap()
}

#[proc_macro]
pub fn rlay(item: TokenStream) -> TokenStream {
    quote! { rlay_core::RlayElement }.into()
}
