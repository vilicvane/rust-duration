#![cfg_attr(docsrs, feature(doc_cfg))]

//! A proc-macro collection that parses human-readable strings at compile time.

use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

/// Parses a human-readable duration string at compile time into an
/// `std::time::Duration` using [humantime].
///
/// ```rust
/// # use std::time::Duration;
/// # use lits::duration;
/// assert_eq!(duration!("7d"), Duration::from_secs(7 * 24 * 60 * 60));
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "humantime")))]
#[cfg(feature = "humantime")]
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
/// assert_eq!(bytes!("1 kiB"), 1024u64);
/// assert_eq!(bytes!("1 kB"), 1000u64);
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "bytesize")))]
#[cfg(feature = "bytesize")]
#[proc_macro]
pub fn bytes(input: TokenStream) -> TokenStream {
  let literal = parse_macro_input!(input as LitStr);
  let string = literal.value();

  match string.parse::<bytesize::ByteSize>() {
    Ok(size) => {
      let bytes = size.as_u64();

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
