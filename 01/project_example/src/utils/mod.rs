// This file defines the "utils" module and its sub-modules, namely "helper".

pub mod helper; // Declare the "helper" sub-module
// When the compiler reads pub mod helper;, it looks for src/network/helper.rs and parses it.

// Public function in "utils" module
pub fn print_hello() {
    println!("Hello from the utils module!");
}
