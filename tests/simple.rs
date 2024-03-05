use ethp::{keccak256, selector};
use hex_literal::hex;

#[test]
fn selector() {
    assert_eq!(hex!("095ea7b3"), selector!("approve(address,uint256)"));
}

#[test]
fn keccak() {
    assert_eq!(
        hex!("8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925"),
        keccak256!("Approval(address,address,uint256)"),
    );
}

#[test]
fn keccak_empty() {
    assert_eq!(
        hex!("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"),
        keccak256!(""),
    );
}
