//! A proc-macro collection that parses human-readable strings at compile time.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::LitInt;
use syn::{LitStr, parse_macro_input};

/// Parses a human-readable duration string at compile time into an
/// `std::time::Duration` using [humantime].
///
/// ```rust
/// # use std::time::Duration;
/// # use lits::duration;
/// const DURATION: Duration = duration!("7d");
/// let duration: Duration = duration!("7d");
///
/// assert_eq!(DURATION, Duration::new(7 * 24 * 60 * 60, 0));
/// assert_eq!(duration, Duration::new(7 * 24 * 60 * 60, 0));
/// ```
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

/// Parses a human-readable byte size string at compile time into an integer
/// using [bytesize].
///
/// ```rust
/// # use lits::bytes;
/// const SIZE: u32 = bytes!("1ki");
/// let size = bytes!("1k");
///
/// assert_eq!(SIZE, 1024);
/// assert_eq!(size, 1000);
/// ```
#[proc_macro]
pub fn bytes(input: TokenStream) -> TokenStream {
  let literal = parse_macro_input!(input as LitStr);
  let string = literal.value();

  match string.parse::<bytesize::ByteSize>() {
    Ok(size) => {
      let bytes = LitInt::new(&size.as_u64().to_string(), Span::call_site());

      quote! {
        #bytes
      }
      .into()
    }
    Err(error) => syn::Error::new(
      literal.span(),
      format!("failed to parse size string: {error}"),
    )
    .to_compile_error()
    .into(),
  }
}
