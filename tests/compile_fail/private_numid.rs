mod module {
    use numid::numid;
    numid!(pub struct TestPublic);
    numid!(struct TestPrivate);
}

fn main() {
    let _npu = module::TestPublic::new();
    let _npr = module::TestPrivate::new(); //~ ERROR struct `TestPrivate` is private
} 
