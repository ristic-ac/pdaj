// lib.rs represents the root of the module tree of a crate
mod example {
    // If pub mod was used here, instead of pub use below, the structure of the module would be known, i.e. it would be possible to call functions from the module with crate_sample::example::add_one
    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let arg = 5;
    /// let answer = crate_sample::example::add_one(arg);
    ///
    /// assert_eq!(6, answer);
    /// ```
    pub fn add_one(x: i32) -> i32 {
        // Ukoliko se pub ne navede, funkcija je vidljiva samo u ovom modulu
        x + 1
    }
}

mod example2 {
    use crate::example::add_one;
    /// Adds any number to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let arg = 5;
    /// let num = 10;
    /// let answer = crate_sample::example2::add_any(arg, num);
    ///
    /// assert_eq!(15, answer);
    /// ```
    pub fn add_any(x: i32, y: i32) -> i32 {
        y + add_one(x) - 1
    }
}

pub use example::add_one; // This way, the function from the module can be used outside the module, by calling it with crate_sample::add_one, i.e. without knowing the structure of the module
pub use example2::add_any;
