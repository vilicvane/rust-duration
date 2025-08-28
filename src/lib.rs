use proc_macro::TokenStream;
use syn::{LitStr, parse_macro_input};
use quote::quote;

#[proc_macro]
pub fn duration(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitStr);
    let s = lit.value();

    match humantime::parse_duration(&s) {
        Ok(dur) => {
            let secs = dur.as_secs();
            let nanos = dur.subsec_nanos();
            let expanded = quote! {
                ::std::time::Duration::new(#secs, #nanos)
            };
            expanded.into()
        }
        Err(e) => {
            let err = syn::Error::new(lit.span(), format!("failed to parse duration literal: {}", e));
            err.to_compile_error().into()
        }
    }
}
