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

Add this to your `Cargo.toml`:

```toml
[dependencies]
numid = "0.2.0"
```

and this to your crate root:

```rust
#[macro_use]
extern crate numid;
```
