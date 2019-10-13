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

# Exemples

```rust
# #[macro_use]
# extern crate numid;

numid!(struct MyId -> 10);

fn main() {
    let id1 = MyId::new();
    let id2 = MyId::new();

    assert!(id2 > id1);
    assert_eq!(id1.value(), 11);
    assert_eq!(id2.value(), 12);
}
```

# Trait implementations

The `Copy`, `Clone`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash` and `Debug`
traits automatically derived for the `struct` using the `derive` attribute.
Additional traits can be derived by providing an explicit `derive` attribute.

The `Display` and `Default` traits are implemented for the `struct`. When
calling `default()`, the struct is initialied with a new value instead of `0`.
*/
 
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
        $(#[$attr])*
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug)]
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
            
            /// Increment the current value and create a new id with value = `current_value()`.
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
            
            /// Return the value of the last id created (with `new()` or `default()`), 
            ///if no id has been created, return `initial_value()`.
            #[allow(dead_code)]
            #[inline]
            pub fn current_value() -> $ty {
                unsafe {
                    *$name::__get_static_mut()
                }
            }
            
            /// return INITIAL_VALUE.
            #[allow(dead_code)]
            #[inline]
            pub const fn initial_value() -> $ty {
                $name::INITIAL_VALUE
            }
            
            /// Replace the current value by the `value` parameter if it superior.
            /// This condition is necessary for not creating multiple ids with the same value.
            /// Return true if the current value has been modified.
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
            
            /// Create a id with a precised value, don't increment the current value.
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
        }
                
        impl Default for $name {
            #[inline]
            fn default() -> $name {
                $name::new()
            }
        }
                
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

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
        assert_eq!(IdWithInitVal::current_value(), IdWithInitVal::initial_value() );
        
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
        let _ = IdWithInitVal::create_lower(150);
    }
}
