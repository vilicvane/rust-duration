//! A tiny proc-macro that parses human-friendly duration strings at compile
//! time using [humantime].
//!
//! The `duration!` macro accepts a string literal and expands to a
//! `std::time::Duration` value constructed with the parsed seconds and
//! nanoseconds. Parsing uses the `humantime` crate, so the same formats are
//! accepted (for example: "1h 30m", "45s", "1500ms").
//!
//! # Examples
//!
//! ```rust
//! use std::time::Duration;
//!
//! use duration::duration;
//!
//! let duration: Duration = duration!("7d");
//!
//! assert_eq!(duration, Duration::new(7 * 24 * 60 * 60, 0));
//! ```
//!
//! # Errors
//!
//! If the provided string literal cannot be parsed by `humantime`, the macro
//! emits a compile-time error describing the parse failure. The macro also
//! requires a string literal as its input; passing arbitrary tokens will fail
//! to parse and result in a compiler error.
//!
//! # Notes
//!
//! - This macro is evaluated at compile time and expands to a const
//!   `std::time::Duration` construction.
//! - The macro intentionally rejects non-literal input to keep the result a
//!   compile-time constant.

use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

/// Parse a human-friendly duration string into a `std::time::Duration` at
/// compile time.
///
/// The input must be a string literal and is parsed using
/// `humantime::parse_duration`. On success the macro expands to
/// `::std::time::Duration::new(seconds, nanos)`. On failure it emits a compile
/// error with the underlying parse message.
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
