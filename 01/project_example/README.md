# Project Structure

This Rust project contains one **package** with multiple **crates** and **modules**.

## Package

A **package** is a bundle of one or more crates managed by Cargo.  

- Defined by **Cargo.toml** (project metadata and dependencies).
- This project is a single package.

## Crates

A **crate** is a compilation unit — it builds into a library or binary.  

- **Library crate:** `src/lib.rs`  
- **Binary crates:**  
  - `src/main.rs` (default executable)  
  - `src/bin/binary1.rs` and `src/bin/binary2.rs` (additional executables)

## Modules

A **module** organizes code within a crate using the `mod` keyword.  

- `network` (in `src/network/mod.rs`) — handles networking logic  
- `utils` (in `src/utils/mod.rs`) — helper and utility functions  

## Submodules

A **submodule** is a nested module defined inside another module’s directory.  

- `network::server` (from `src/network/server.rs`)  
- `utils::helper` (from `src/utils/helper.rs`)

Each submodule is declared in its parent’s `mod.rs` file, allowing hierarchical organization of code.
