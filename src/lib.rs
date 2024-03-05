#![doc = include_str!("../README.md")]
#![warn(missing_docs, rustdoc::all)]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};
use tiny_keccak::{Hasher, Keccak};

/// Computes the [Keccak-256](https://keccak.team/files/Keccak-submission-3.pdf) hash of a string
/// literal at compile time.
///
/// # Examples
///
/// ```
/// use hex_literal::hex;
/// use ethp::keccak256;
///
/// assert_eq!(
///     keccak256!("transfer(address,uint256)"),
///     hex!("a9059cbb2ab09eb219583f4a59a5d0623ade346d962bcd4e46b11da047c9049b"),
/// );
/// ```
#[proc_macro]
pub fn keccak256(tokens: TokenStream) -> TokenStream {
    let mut hash = [0u8; 32];
    let mut sha3 = Keccak::v256();
    let to_hash = parse_macro_input!(tokens as LitStr).value();
    sha3.update(to_hash.as_bytes());
    sha3.finalize(&mut hash);
    quote! { [#( #hash, )*] }.into()
}

/// Computes the first four bytes of the
/// [Keccak-256](https://keccak.team/files/Keccak-submission-3.pdf) hash of the provided string
/// literal at compile time.
///
/// # Examples
///
/// ```
/// use hex_literal::hex;
/// use ethp::selector;
///
/// assert_eq!(selector!("transfer(address,uint256)"), hex!("a9059cbb"));
/// ```
#[proc_macro]
pub fn selector(tokens: TokenStream) -> TokenStream {
    let mut selector = [0u8; 4];
    let mut sha3 = Keccak::v256();
    let to_hash = parse_macro_input!(tokens as LitStr).value();
    sha3.update(to_hash.as_bytes());
    sha3.finalize(&mut selector);
    quote! { [#( #selector, )*] }.into()
}

/// Alias for [`keccak256!(..)`](keccak256!) for improved readability when working with event logs.
///
/// # Examples
///
/// ```
/// use hex_literal::hex;
/// use ethp::{event, keccak256};
///
/// let exp = hex!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");
/// let hash = keccak256!("Transfer(address,address,uint256)");
/// let event_sig = event!("Transfer(address,address,uint256)");
/// assert_eq!(exp, hash);
/// assert_eq!(exp, event_sig);
/// ```
#[proc_macro]
pub fn event(tokens: TokenStream) -> TokenStream {
    keccak256(tokens)
}
