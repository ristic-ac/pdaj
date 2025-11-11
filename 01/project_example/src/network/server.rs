use crate::utils::helper; // This line is not necessary but shows the relationship. We use crate::, because both modules are within the same crate (library).

// A function in the "server" module
pub fn start_server() {
    println!("Server started from the network::server module!");
}
