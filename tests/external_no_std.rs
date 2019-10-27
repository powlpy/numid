#![no_std]
use numid::numid;

numid!(struct Test);

#[test]
fn external_no_std() {
    let id = Test::new();
    assert!(id.value() != 0);
}
