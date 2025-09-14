#![cfg_attr(docsrs, feature(doc_cfg))]

//! A proc-macro collection that parses human-readable strings at compile time.

use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

#[cfg(feature = "humantime")]
mod duration;

/// Parses a human-readable duration string at compile time into an
/// `std::time::Duration` using [humantime].
///
/// ```rust
/// # use std::time::Duration;
/// # use lits::duration;
/// assert_eq!(duration!("7d"), Duration::from_secs(7 * 24 * 60 * 60));
/// assert_eq!(duration!("1d" * 7), Duration::from_secs(7 * 24 * 60 * 60));
/// assert_eq!(duration!("2s" / 20), Duration::from_millis(100));
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "humantime")))]
#[cfg(feature = "humantime")]
#[proc_macro]
pub fn duration(input: TokenStream) -> TokenStream {
  use crate::duration::DurationMacroInput;

  let DurationMacroInput { duration, scalar } = parse_macro_input!(input);

  match humantime::parse_duration(&duration.value()) {
    Ok(duration) => {
      let duration = duration.mul_f64(scalar);

      let seconds = duration.as_secs();
      let nanoseconds = duration.subsec_nanos();

      quote! {
          ::std::time::Duration::new(#seconds, #nanoseconds)
      }
      .into()
    }
    Err(error) => syn::Error::new(
      duration.span(),
      format!("failed to parse string as Duration: {error}"),
    )
    .to_compile_error()
    .into(),
  }
}

/// Parses a human-readable date-time string at compile time into an
/// `std::time::SystemTime` using [humantime].
///
/// ```rust
/// # use std::time::{Duration, SystemTime, UNIX_EPOCH};
/// # use lits::datetime;
/// assert_eq!(
///   datetime!("2000-01-01T00:00:00Z"),
///   UNIX_EPOCH + Duration::from_secs(946684800)
/// );
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "humantime")))]
#[cfg(feature = "humantime")]
#[proc_macro]
pub fn datetime(input: TokenStream) -> TokenStream {
  let literal: LitStr = parse_macro_input!(input);
  let string = literal.value();

  match humantime::parse_rfc3339_weak(&string) {
    Ok(datetime) => {
      use std::time::UNIX_EPOCH;

      // Humantime doesn't support time earlier than epoch anyway, so we can
      // safely unwrap here if it was an Ok.
      let duration = datetime.duration_since(UNIX_EPOCH).unwrap();

      let seconds = duration.as_secs();
      let nanoseconds = duration.subsec_nanos();

      quote! {
        ::std::time::SystemTime::UNIX_EPOCH + ::std::time::Duration::new(#seconds, #nanoseconds)
      }
      .into()
    }
    Err(error) => syn::Error::new(
      literal.span(),
      format!("failed to parse string as SystemTime: {error}"),
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
  let literal: LitStr = parse_macro_input!(input);
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
      format!("failed to parse string as byte size: {error}"),
    )
    .to_compile_error()
    .into(),
  }
}
