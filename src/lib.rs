use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn duration(input: TokenStream) -> TokenStream {
    let literal = parse_macro_input!(input as LitStr);
    let string = literal.value();

    match humantime::parse_duration(&string) {
        Ok(duration) => {
            let seconds = duration.as_secs();
            let nanoseconds = duration.subsec_nanos();

            quote! {
                ::std::time::Duration::new(#seconds, #nanoseconds)
            }
            .into()
        }
        Err(error) => syn::Error::new(
            literal.span(),
            format!("failed to parse duration string: {error}"),
        )
        .to_compile_error()
        .into(),
    }
}
