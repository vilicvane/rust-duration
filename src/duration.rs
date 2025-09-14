use syn::{
  LitFloat, LitInt, LitStr, Token,
  parse::{Parse, ParseStream},
};

pub struct DurationMacroInput {
  pub duration: LitStr,
  pub scalar: f64,
}

impl Parse for DurationMacroInput {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let duration: LitStr = input.parse()?;

    let scalar = if input.is_empty() {
      1.0
    } else {
      let reciprocal = {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![*]) {
          input.parse::<Token![*]>()?;
          false
        } else if lookahead.peek(Token![/]) {
          input.parse::<Token![/]>()?;
          true
        } else {
          return Err(lookahead.error());
        }
      };

      let number = {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitInt) {
          input.parse::<LitInt>()?.base10_parse::<f64>()?
        } else if lookahead.peek(LitFloat) {
          input.parse::<LitFloat>()?.base10_parse::<f64>()?
        } else {
          return Err(lookahead.error());
        }
      };

      if reciprocal { 1.0 / number } else { number }
    };

    Ok(Self { duration, scalar })
  }
}
