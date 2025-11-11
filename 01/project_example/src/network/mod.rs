// This file defines the "network" module and its sub-modules, namely "server".

pub mod server; // Declare the "server" sub-module
// When the compiler reads pub mod server;, it looks for src/network/server.rs and parses it.

// Public function in the "network" module
pub fn connect() {
    println!("Connecting to the network...");
}
