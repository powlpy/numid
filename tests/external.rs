use numid::numid;

numid!(struct Test);

#[test]
fn external() {
    let id = Test::new();
    assert!(id.value() != 0);
}
