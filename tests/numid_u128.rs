use numid::numid;
// rustc v1.26+
numid!(struct Id128(u128) -> 1u128 << 100);

#[test]
fn tests_u128() {
    let id = Id128::new();
    assert_eq!(id.value(), (1u128 << 100) + 1);

    assert!(Id128::replace_current_value(1u128 << 110));
    assert_eq!(Id128::replace_current_value(1u128 << 108), false);

    let _ = Id128::create_lower(1u128 << 80);
}

#[test]
fn tests_const_create_lower_u128() {
    const _C0: Id128 = Id128::const_create_lower(0u128);
    const _C1: Id128 = Id128::const_create_lower(1u128 << 64 + 1);
    const _C2: Id128 = Id128::const_create_lower(1u128 << 100);
}
