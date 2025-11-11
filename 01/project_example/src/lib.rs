pub mod network; // Declare the "network" module, defined in "network/mod.rs"
pub mod utils; // Declare the "utils" module, defined in "utils/mod.rs"
// These lines tell the Rust compiler to look for the module definitions in their respective files, and allow the main crate to access them.
// Removing `pub` would make the modules private to the crate and inaccessible from outside, which in turn means the main binary crate would not be able to use them, i.e., it would cause compilation errors in `src/main.rs`.
