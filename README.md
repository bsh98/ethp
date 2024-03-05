# ethp

A collection of Rust [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html) for evaluating common Ethereum and blockchain related functions at compile time.

## Example

```rust
use hex_literal::hex;
use ethp::{keccak256, selector};

assert_eq!(
    keccak256!("Transfer(address,address,uint256)"),
    hex!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"),
);
assert_eq!(selector!("transfer(address,uint256)"), hex!("a9059cbb"));
```
