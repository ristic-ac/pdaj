use crate::network; // This line is not necessary but shows the relationship. We use crate::, because both modules are within the same crate (library).

// A helper function within the "helper" module
pub fn print_help() {
    println!("Helper function from the utils::helper module!");
}
