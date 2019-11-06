
use numid::numid;

numid!(struct Test(u128) -> 1u128 << 65);

fn main() {
    const _CONST: Test = Test::const_create_lower(1u128 << 66); //~ ERROR any use of this value will cause an error
}
 
 
