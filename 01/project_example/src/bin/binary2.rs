use project_example::utils::helper; // We use project_example because this is a binary crate depending on the library crate named "project_example". Therefore, we access the modules via the crate name.

fn main() {
    println!("This is binary 2.");
    helper::print_help();
}
