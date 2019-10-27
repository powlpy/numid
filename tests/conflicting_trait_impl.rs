#![no_std]

use numid::numid;

#[allow(unused_imports)]
use core::fmt::Display;

numid!(struct Test);

#[test]
fn conflicting_trait_impl() {} 
