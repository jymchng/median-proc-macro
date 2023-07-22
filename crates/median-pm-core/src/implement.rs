use proc_macro::{TokenStream, TokenTree};
use proc_macro2::{TokenStream as TokenStream2, Span};
use quote::quote;
use syn::{LitFloat, LitInt};
use proc_macro_error::emit_error;

fn parse_tokenstream(input: TokenStream) -> Vec<u64> {
  let mut numbers = vec![];
  for token in input.into_iter() {
    match token {
      TokenTree::Literal(a) => {
        let span = a.span();
        let source = a.to_string();
        let num_ident = source.parse::<u64>();
        if let Ok(num) = num_ident {
          numbers.push(num);
        } else {
          emit_error!(span, "`{}` cannot be parsed into a value of type `u64`", source);
        }
      },
      TokenTree::Punct(_) => {},
      TokenTree::Group(g) => {
        let span = g.span();
        let source = g.to_string();
        emit_error!(span, "`{}` cannot be parsed into a value of type `u64` or into a punctuation, e.g. `,`", source);
      },
      TokenTree::Ident(i) => {
        let span = i.span();
        let source = i.to_string();
        emit_error!(span, "`{}` cannot be parsed into a value of type `u64` or into a punctuation, e.g. `,`", source);
      }
    }
  }
  numbers
}

fn calculate_median_return_f64(mut numbers: Vec<u64>) -> f64 {
  numbers.sort();

  let n = numbers.len();
  if n == 0 {
      return 0.0;
  }

  let middle1 = numbers[n / 2 - 1];
  let middle2 = numbers[n / 2];
  (middle1 + middle2) as f64 / 2.0
}

fn calculate_median_return_u64(mut numbers: Vec<u64>) -> u64 {
  numbers.sort();
  let n = numbers.len();
  numbers[n / 2]
}


fn convert_to_float_if_integer(input: &str) -> String {
  if input.chars().all(|c| c.is_digit(10)) {
      format!("{}.0", input)
  } else {
      input.to_string()
  }
}


pub fn median_impl(input: TokenStream) -> TokenStream2 {
  let punctuated_integers = parse_tokenstream(input);
  let returned_ts = if punctuated_integers.len() % 2 == 0 {
    let median = calculate_median_return_f64(punctuated_integers).to_string();
    let median = convert_to_float_if_integer(&median);
    let median_litfloat = LitFloat::new(median.as_str(), Span::call_site());
    quote!(
      #median_litfloat
    )
  } else {
    let median = calculate_median_return_u64(punctuated_integers).to_string();
    let median_litint = LitInt::new(median.as_str(), Span::call_site());
    quote!(
      #median_litint
    )
  };
  returned_ts
}