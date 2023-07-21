use proc_macro::{TokenStream, TokenTree, Literal, Punct};
use proc_macro2::{TokenStream as TokenStream2, Ident};
use quote::quote;
use syn::{parse_macro_input, Result, Token, LitFloat};
use syn::parse::{Parse, ParseStream};
use syn::{parse_str, punctuated::Punctuated, LitInt};
use proc_macro_error::emit_error;

fn parse_tokenstream(input: TokenStream) -> Vec<u64> {
  let mut numbers = vec![];
  for token in input.into_iter() {
    match token {
      TokenTree::Literal(a) => {
        let span = a.span();
        let source = a.to_string();
        let num_ident = source.parse::<u64>();
        if num_ident.is_err() {
          emit_error!(span, "`{}` cannot be parsed into a value of type `u64`", source);
        } else {
          numbers.push(num_ident.unwrap());
        }
      },
      TokenTree::Punct(p) => {},
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

pub fn median_impl(input: TokenStream) -> TokenStream2 {
  eprintln!("INPUT: {:#?}", input);
  let punctuated_integers = parse_tokenstream(input);
  let returned_ts = if punctuated_integers.len() % 2 == 0 {
    let median = calculate_median_return_f64(punctuated_integers).to_string();
    let median_litfloat = parse_str::<LitFloat>(median.as_str());
    match median_litfloat {
      Ok(ref mfloat) => {},
      Err(ref err) => {
        emit_error!("Unable to convert `{}` to a value of type `LitFloat`", median)
      }
    }
    let mfloat = median_litfloat.unwrap();
    eprintln!("{:#?}", mfloat);
    quote!(
      #mfloat
    ).into()
  } else {
    let median = calculate_median_return_u64(punctuated_integers).to_string();
    let median_litint = parse_str::<LitInt>(median.as_str());
    match median_litint {
      Ok(ref mint) => {},
      Err(ref err) => {
        emit_error!("Unable to convert `{}` to a value of type `LitInt`", median);
      }
    }
    let mint = median_litint.unwrap();
    eprintln!("{:#?}", mint);
    quote!(
      #mint
    ).into()
  };
  returned_ts
}

// Use quote to generate a TokenStream that represents the Vec<u64>
// let expanded = quote! {
//   {
//       let mut vec = Vec::new();
//       #( vec.push(#numbers); )*
//       vec
//   }
// };