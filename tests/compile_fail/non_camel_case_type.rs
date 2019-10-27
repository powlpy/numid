
#![deny(warnings)]

use numid::numid;

numid!(struct test); //~ ERROR type `test` should have an upper camel case name

fn main() {}
 
