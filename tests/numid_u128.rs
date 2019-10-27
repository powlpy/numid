use numid::numid;
numid!(struct Id128(u128) -> 1u128 << 100);

// rustc v1.26+
#[test]
fn tests_u128() {
    let id = Id128::new();
    assert_eq!(id.value(), (1u128 << 100) + 1);

    assert!(Id128::replace_current_value(1u128 << 110));
    assert_eq!(Id128::replace_current_value(1u128 << 108), false);

    let _ = Id128::create_lower(1u128 << 80);
}
