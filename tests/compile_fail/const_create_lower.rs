
use numid::numid;

numid!(struct Test -> 100);

fn main() {
    const _CONST: Test = Test::const_create_lower(101); //~ ERROR any use of this value will cause an error
}
 
