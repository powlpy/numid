# numid

This crate provide the `numid!` macro for generating structures which behave like numerical id.

[![Build Status](https://travis-ci.com/powlpy/numid.svg?branch=master)](https://travis-ci.com/powlpy/numid)
[![Crate](https://img.shields.io/crates/v/numid.svg)](https://crates.io/crates/numid)
[![Documentation](https://docs.rs/numid/badge.svg)](https://docs.rs/numid)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.31+-yellow.svg)
[![License](https://img.shields.io/crates/l/numid.svg)](https://github.com/powlpy/numid/blob/master/LICENSE)

## Example

```rust
use numid::numid;

numid!(pub struct MyId -> 10);

fn main() {
    let id1 = MyId::new();
    let id2 = MyId::new();

    assert!(id2 > id1);
    assert_eq!(id1.value(), 11);
    assert_eq!(id2.value(), 12);
}
```

## Usage

Add this to your `Cargo.toml` :

```toml
[dependencies]
numid = "0.2"
```

You can now create all the ids you want in your programs :

```rust
use numid::numid;

numid!(struct Id); // basic id
numid!(pub struct _Id2); // public
numid!(pub(crate) struct _Id3); // specific public
numid!(#[doc(hidden)] struct _Id4); // with attribut
numid!(struct _Id5 -> 100); // init const specified
numid!(struct _Id6(u128)); // type specified
numid!(#[doc(hidden)] pub struct _Id7(u32) -> 10); // all the thing you can want

fn main() { 
    let id1 = Id::new();
    let id2 = Id::default();
    println!("id1 : {:?}", id1); // id1 : Id(1)
    println!("id2 : {}", id2); // id2 : 2
    const _CONST : u64 = Id::initial_value();
    assert_eq!(Id::current_value(), 2);
    assert!(Id::replace_current_value(50));
    let _id0 = Id::create_lower(0);
}
```
Consult the [documentation](https://docs.rs/numid) for more informations.
