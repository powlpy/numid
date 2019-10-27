#![deny(warnings)]

use numid::numid;

numid!(#[allow(non_camel_case_types)] struct test);

#[test]
fn allow_non_camel_case_types() {
    let id = test::new();
    assert!(id.value() != 0);
}
