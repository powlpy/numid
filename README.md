# numid

This crate provide the `numid!` macro for generating structures which behave like numerical id.

## Example

```rust
# #[macro_use]
# extern crate numid;

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
numid = "1.0"
```

and this to your crate root:

```rust
#[macro_use]
extern crate numid;
```


