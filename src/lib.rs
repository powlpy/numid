// Copyright 2019 numid Developers
//
// Licensed under the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>. This file may not be
// copied, modified, or distributed except according to those terms.

/*!
This crate provide the `numid!` macro for creating numerical id.

# Syntax

```ignore
numid!([pub] struct NAME [(TYPE)] [-> CONSTANT]);
```
If not indicated, TYPE=`u64` and CONSTANT=`0`.

# Attributes

Attributes can be attached to the generated `struct` by placing them
before the `struct` keyword (or `pub` if public).

# Examples

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

See [`example`](./example/index.html) for documentation of code generated by `numid!`.

# Trait implementations

The `Copy`, `Clone`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash` and `Debug`
traits automatically derived for the `struct` using the `derive` attribute.
Additional traits can be derived by providing an explicit `derive` attribute.

The `Display`, `Binary`, `Octal`, `LowerHex`, `UpperHex` and `Default` traits are implemented for
the `struct`. When calling `default()`, the struct is initialized with a new value instead of `0`.
Your own version of `Display` can be implemented by disabling the `display` feature.

# Methods

The following methods are defined for the generated `struct` (only `value` and `reproduce` need a instance) :

- `new` : create a new id
- `value` : get the id value
- `current_value` : get the value of the last id or initial_value if no id created
- `initial_value` : get the value defined when calling `numid!`
- `replace_current_value` : see  [`example::NumId::replace_current_value`](./example/struct.NumId.html#method.replace_current_value)
- `create_maybe` : see  [`example::NumId::create_maybe`](./example/struct.NumId.html#method.create_maybe)
- `create_lower` : see  [`example::NumId::create_lower`](./example/struct.NumId.html#method.create_lower)
- `const_create_lower` : see [`example::NumId::const_create_lower`](./example/struct.NumId.html#method.const_create_lower)
- `reproduce` : see [`example::NumId::reproduce`](./example/struct.NumId.html#method.reproduce)

See [`example::NumId`](./example/struct.NumId.html) for more documentation of  methods generated by `numid!`.

# Crate feature

This crate provides the `display` feature enabled by default who automatically implemente the `Display` trait
in the structure generated by the `numid!` macro.  If you want to implemente your own version of `Display`,
add `default-features = false` in the `dependencies.numid` section of your `Cargo.toml`.
*/

#![cfg_attr(not(test), no_std)]
#![doc(html_root_url = "https://docs.rs/numid")]
#![warn(missing_docs)]

/*
Features used in this crate by rust version :
 - 1.31 : const fn
 - 1.30 : $vis
 - 1.20 : associated const

Current minimum rust version of the crate : 1.31
*/

#[doc(hidden)]
pub extern crate const_fn_assert as _const_fn_assert;

#[doc(hidden)]
pub extern crate core as _core;

/// # Examples
/// ```
/// use numid::numid;
///
/// numid!(struct Id); // basic id
/// numid!(pub struct Id2); // public
/// numid!(pub(crate) struct Id3); // restricted public
/// numid!(#[doc(hidden)] struct Id4); // with attribut
/// numid!(struct Id5 -> 100); // init const specified
/// numid!(struct Id6(u128)); // type specified
/// numid!(#[doc(hidden)] pub struct Id7(u32) -> 10); // all the thing you can want
/// ```
#[macro_export]
macro_rules! numid {
    ($(#[$attr:meta])* $vis:vis struct $name:ident) => {
        numid!{$(#[$attr])* $vis struct $name(u64) -> 0}
    };
    ($(#[$attr:meta])* $vis:vis struct $name:ident -> $init_val:expr) => {
        numid!{$(#[$attr])* $vis struct $name(u64) -> $init_val}
    };
    ($(#[$attr:meta])* $vis:vis struct $name:ident($ty:ty)) => {
        numid!{$(#[$attr])* $vis struct $name($ty) -> 0}
    };
    ($(#[$attr:meta])* $vis:vis struct $name:ident($ty:ty) -> $init_val:expr) => {

        /// A numerical id generated with the `numid!` macro.
        #[warn(non_camel_case_types)]
        //#[warn(dead_code)] // rust-lang : issue 66030
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug)]
        $(#[$attr])*
        $vis struct $name($ty);

        impl $name {
            /// Constant defined when calling the `numid!` macro (0 if not defined).
            /// The firt id created (with `new()` or `default()`) will have value = `INITIAL_VALUE + 1`.
            pub const INITIAL_VALUE: $ty = $init_val;

            #[doc(hidden)]
            #[inline]
            unsafe fn __get_static_mut() -> &'static mut $ty {
                static mut CURRENT_VALUE: $ty = $name::INITIAL_VALUE;
                &mut CURRENT_VALUE
            }

            /// Increment the "current value" and create a new id with value = `current_value()`.
            #[allow(dead_code)]
            #[inline]
            pub fn new() -> $name {
                $name(unsafe {
                    let v = $name::__get_static_mut();
                    *v += 1;
                    *v
                })
            }

            /// Get the value of the id.
            #[allow(dead_code)]
            #[inline]
            pub const fn value(self) -> $ty {
                self.0
            }

            /// Return the "current value", the value of the last id created (with `new()`,
            /// `default()` or `create_maybe()`),
            /// if no id has been created, the "current value" equal `initial_value()`.
            #[allow(dead_code)]
            #[inline]
            pub fn current_value() -> $ty {
                unsafe {
                    *$name::__get_static_mut()
                }
            }

            /// Return INITIAL_VALUE.
            #[allow(dead_code)]
            #[inline]
            pub const fn initial_value() -> $ty {
                $name::INITIAL_VALUE
            }

            /// Replace the "current value" by the `value` parameter if it superior.
            /// This condition is necessary for not creating multiple ids with the same value.
            /// Return true if the "current value" has been modified.
            #[allow(dead_code)]
            pub fn replace_current_value(value: $ty) -> bool {
                let cond = value > $name::current_value();
                if cond {
                    unsafe {
                        let v = $name::__get_static_mut();
                        *v = value;
                    }
                }
                cond
            }

            /// Return Some id with specified value and replace the "current value" if
            /// `replace_current_value(value)` is `true`.
            /// Return None otherwise.
            #[allow(dead_code)]
            #[inline]
            pub fn create_maybe(value: $ty) -> Option<$name> {
                if $name::replace_current_value(value) {
                    Some($name(value))
                } else {
                    None
                }
            }

            /// Create a id with a precised value, don't increment the "current value".
            /// The value must be inferior or equal as `INITIAL_VALUE` for not
            /// interfering with the id system.
            ///
            /// # Panics
            /// panic if `value > INITIAL_VALUE`
            #[allow(dead_code)]
            #[inline]
            pub fn create_lower(value: $ty) -> $name {
                assert!(value <= $name::initial_value());
                $name(value)
            }

            /// Const version of [`create_lower`](#method.create_lower),
            /// can be used in a const environnement. In a non-const environnement, give a
            /// `index: out of bounds` panic message, `create_lower` giving a more descriptive
            /// message is therefore preferred.
            ///
            /// # Panics
            /// panic if `value > INITIAL_VALUE`
            #[allow(dead_code)]
            #[inline]
            pub const fn const_create_lower(value: $ty) -> $name {
                $crate::_const_fn_assert::cfn_assert!(value <= $name::initial_value());
                $name(value)
            }

            /// Return a copy of the id if the value is inferior or equal as `initial_value`,
            /// else return a new id and update the `current_value`.
            #[allow(dead_code)]
            #[inline]
            pub fn reproduce(self) -> $name {
                if self.0 > $name::initial_value() {
                    $name::new()
                } else {
                    self
                }
            }
        }

        /// Increment the "current value" and create a new id with value = `current_value()`.
        /// This is equivalent to `new()`.
        impl Default for $name {
            #[inline]
            fn default() -> $name {
                $name::new()
            }
        }

        $crate::__fmt_impl_numid!($name : Binary, Octal, LowerHex, UpperHex);
        $crate::__display_numid!($name);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __fmt_impl_numid {
    ($name:ident : $($trait:ident),+) => {
        $(
            impl $crate::_core::fmt::$trait for $name {
                fn fmt(&self, f: &mut $crate::_core::fmt::Formatter<'_>) -> $crate::_core::fmt::Result {
                    $crate::_core::fmt::$trait::fmt(&self.0, f)
                }
            }
        )+
    }
}

#[cfg(feature = "display")]
#[macro_export]
#[doc(hidden)]
macro_rules! __display_numid {
    ($name:ident) => {
        $crate::__fmt_impl_numid!($name: Display);
    };
}

#[cfg(not(feature = "display"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __display_numid {
    ($name:ident) => {};
}

#[cfg(feature = "example")]
pub mod example;

// Always test the example.
#[cfg(all(test, not(feature = "example")))]
mod example;

#[cfg(test)]
mod tests {

    numid!(struct Id);
    numid!(struct IdWithInitVal -> 100);
    numid!(struct UnusedId(u32) -> 10);

    #[test]
    fn tests_id_used() {
        assert_eq!(Id::initial_value(), 0);
        assert_eq!(Id::current_value(), Id::initial_value());

        let id0 = Id::new();

        assert_eq!(Id::current_value(), 1);
        assert_eq!(id0.value(), Id::current_value());

        let id1 = Id::new();
        assert_eq!(Id::current_value(), 2);
        assert_eq!(id1.value(), Id::current_value());
        assert_ne!(id0, id1);
        assert!(id1 > id0);

        let id2 = Id::default();
        assert_eq!(Id::current_value(), 3);
        assert_eq!(id2.value(), Id::current_value());
        assert!(id2 > id1);

        assert!(Id::replace_current_value(10));
        assert_eq!(Id::current_value(), 10);
        assert_eq!(Id::replace_current_value(1), false);
        assert_eq!(Id::current_value(), 10);
        let id3 = Id::new();
        assert_eq!(id3.value(), 11);
    }

    #[test]
    fn tests_id_with_init_val_used() {
        assert_eq!(IdWithInitVal::initial_value(), 100);
        assert_eq!(
            IdWithInitVal::current_value(),
            IdWithInitVal::initial_value()
        );

        let id0 = IdWithInitVal::new();

        assert_eq!(IdWithInitVal::current_value(), 101);
        assert_eq!(id0.value(), IdWithInitVal::current_value());

        let id1 = IdWithInitVal::new();
        assert_eq!(IdWithInitVal::current_value(), 102);
        assert_eq!(id1.value(), IdWithInitVal::current_value());
        assert!(id1 > id0);

        assert!(IdWithInitVal::replace_current_value(150));
        assert_eq!(IdWithInitVal::current_value(), 150);
        assert_eq!(IdWithInitVal::replace_current_value(1), false);
        assert_eq!(IdWithInitVal::current_value(), 150);
        let id2 = IdWithInitVal::new();
        assert_eq!(id2.value(), 151);
    }

    #[test]
    fn tests_create_lower() {
        let id = Id::create_lower(0);
        assert_eq!(id.value(), 0);

        let _ = IdWithInitVal::create_lower(0);
        let _ = IdWithInitVal::create_lower(50);
        let _ = IdWithInitVal::create_lower(100);

        let _ = UnusedId::create_lower(5);
    }

    #[test]
    #[should_panic]
    fn tests_create_lower_fail() {
        let _ = IdWithInitVal::create_lower(101);
    }

    #[test]
    fn test_const_create_lower() {
        const C: Id = Id::const_create_lower(0);
        assert_eq!(C.value(), 0);

        let _ = IdWithInitVal::const_create_lower(0);
        const _C0: IdWithInitVal = IdWithInitVal::const_create_lower(50);
        const _C1: IdWithInitVal = IdWithInitVal::const_create_lower(100);

        const _C2: UnusedId = UnusedId::const_create_lower(5);
    }

    #[test]
    #[should_panic]
    fn tests_const_create_lower_fail() {
        //non-const test
        let _ = IdWithInitVal::const_create_lower(101);
    }

    #[test]
    fn tests_create_maybe() {
        numid!(struct IdMaybe -> 1);

        assert_eq!(IdMaybe::create_maybe(5).unwrap().value(), 5);
        assert_eq!(IdMaybe::current_value(), 5);
        assert_eq!(IdMaybe::create_maybe(3), None);
        assert_eq!(IdMaybe::current_value(), 5);
        assert_eq!(IdMaybe::create_maybe(5), None);
        assert_eq!(IdMaybe::create_maybe(8).unwrap().value(), 8);
    }

    #[test]
    fn tests_reproduce() {
        numid!(struct Id -> 10);

        let id0 = Id::new();
        let id1 = Id::create_lower(5);
        let id2 = Id::create_lower(10);

        let id3 = id0.reproduce();

        assert!(id0 < id3);
        assert_eq!(id1, id1.reproduce());
        assert_eq!(id2, id2.reproduce());
        assert!(id3 < id0.reproduce());
    }

    #[test]
    fn test_debug() {
        numid!(struct IdDebug);

        let id = IdDebug::new();
        assert_eq!(format!("{:?}", id), "IdDebug(1)");

        numid!(struct IdDebugHexa -> 0x1b3d - 1);

        let idh = IdDebugHexa::new();
        assert_eq!(format!("{:x?}", idh), "IdDebugHexa(1b3d)");
        assert_eq!(format!("{:X?}", idh), "IdDebugHexa(1B3D)");
    }

    macro_rules! test_fmt {
        // $literal introduced in rustc 1.32
        ($func:ident, $fmt:tt, $fmt2:tt, $value:expr, $fmt_repr:expr, $repr:expr) => {
            #[test]
            fn $func() {
                numid!(struct Test -> $value - 1);

                let id = Test::new();
                assert_eq!(format!($fmt, id), $repr);
                assert_eq!(format!($fmt2, id), concat!($fmt_repr, $repr));
            }
        };
    }

    test_fmt!(test_binary, "{:b}", "{:#b}", 0b1010, "0b", "1010");
    test_fmt!(test_octal, "{:o}", "{:#o}", 0o7706, "0o", "7706");
    test_fmt!(test_lowerhex, "{:x}", "{:#x}", 0xEFFF, "0x", "efff");
    test_fmt!(test_upperhex, "{:X}", "{:#X}", 0xEFFF, "0x", "EFFF");

    #[cfg(feature = "display")]
    #[test]
    fn test_display() {
        numid!(struct IdDisplay);

        let id = IdDisplay::new();
        assert_eq!(format!("{}", id), "1");
    }

    mod submodule {
        numid!(pub struct PublicId);
        numid!(struct PrivateId);

        #[test]
        fn test_private() {
            let _ = PrivateId::new();
        }
    }

    #[test]
    fn test_public() {
        let _ = submodule::PublicId::new();
    }

    #[test]
    fn test_pub_crate() {
        mod module {
            numid!(pub (crate) struct Test -> 4);
        }

        assert_eq!(module::Test::current_value(), 4);
    }

    #[test]
    fn test_pub_in_module() {
        mod module {
            mod submodule {
                numid!(
                    // `pub (in super)` means only the module `module` will
                    // be able to access this.
                    pub (in super) struct Test -> 7
                );
            }

            mod test {
                // Note: due to `pub (in super)`,
                // this cannot be accessed directly by the testing code.
                pub(super) fn value() -> u64 {
                    super::submodule::Test::current_value()
                }
            }

            pub fn value() -> u64 {
                test::value()
            }
        }

        assert_eq!(module::value(), 7);
    }
}
