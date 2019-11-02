# numid

[![Build Status](https://travis-ci.com/powlpy/numid.svg?branch=master)](https://travis-ci.com/powlpy/numid)
[![Crate](https://img.shields.io/crates/v/numid.svg)](https://crates.io/crates/numid)
[![Documentation](https://docs.rs/numid/badge.svg)](https://docs.rs/numid)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.31+-yellow.svg)
[![License](https://img.shields.io/crates/l/numid.svg)](https://github.com/powlpy/numid/blob/master/LICENSE)

This crate provide the `numid!` macro for generating structures which behave like numerical id.

- [Documentation](https://docs.rs/numid)
- [Release notes](https://github.com/powlpy/numid/releases)

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
numid!(pub struct Id2); // public
numid!(pub(crate) struct Id3); // restricted public
numid!(#[doc(hidden)] struct Id4); // with attribut
numid!(struct Id5 -> 100); // init const specified
numid!(struct Id6(u128)); // type specified
numid!(#[doc(hidden)] pub struct Id7(u32) -> 10); // all the thing you can want
```
Consult the [documentation](https://docs.rs/numid) for more information.

## TODO list

 - `serde` feature
