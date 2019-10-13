//! This module shows an example of code generated by the macro. 
//! **IT MUST NOT BE USED OUTSIDE THIS CRATE**.

/// A numerical id.
/// Generated with the macro call `numid!(struct NumId);`.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug)]
struct NumId(u64);
        
impl NumId {
    /// Constant defined when calling the `numid!` macro (0 if not defined).
    /// The firt id created (with `new()` or `default()`) will have value = `INITIAL_VALUE + 1`.
    pub const INITIAL_VALUE: u64 = 0;
            
    #[doc(hidden)]
    #[inline]
    unsafe fn __get_static_mut() -> &'static mut u64 {
        static mut CURRENT_VALUE: u64 = NumId::INITIAL_VALUE;
        &mut CURRENT_VALUE
    }
    
    /// Increment the current value and create a new id with value = `current_value()`.
    #[allow(dead_code)]
    #[inline]
    pub fn new() -> NumId {
        NumId(unsafe {
            let v = NumId::__get_static_mut();
            *v += 1;
            *v
        })
    }
    
    /// Get the value of an id.
    #[allow(dead_code)]
    #[inline]
    pub const fn value(self) -> u64 {
        self.0
    }
    
    /// Return the value of the last id created (with `new()` or `default()`), 
    ///if no id has been created, return `initial_value()`.
    #[allow(dead_code)]
    #[inline]
    pub fn current_value() -> u64 {
        unsafe {
            *NumId::__get_static_mut()
        }
    }
    
    /// return INITIAL_VALUE.
    #[allow(dead_code)]
    #[inline]
    pub const fn initial_value() -> u64 {
        NumId::INITIAL_VALUE
    }
    
    /// Replace the current value by the `value` parameter if it superior.
    /// This condition is necessary for not creating multiple ids with the same value.
    /// Return true if the current value has been modified.
    #[allow(dead_code)]
    pub fn replace_current_value(value: u64) -> bool {
        let cond = value > NumId::current_value();
        if cond {
            unsafe {
                let v = NumId::__get_static_mut();
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
    pub fn create_lower(value: u64) -> NumId {
        assert!(value <= NumId::initial_value());
        NumId(value)
    }
}
        
impl Default for NumId {
    #[inline]
    fn default() -> NumId {
        NumId::new()
    }
}
        
impl std::fmt::Display for NumId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
