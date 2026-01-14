use std::env;
use std::fs;

fn main() {
    // Collect command line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Ensure the correct number of arguments is provided
    if args.len() < 3 {
        eprintln!("Usage: minigrep <query> <file_path>");
        std::process::exit(1);
    }

    // Assign arguments to specific variables
    let executable_name = &args[0];
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for '{}'", query);
    println!("In file '{}'", file_path);

    // Read the file content
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    // Search for the query in the file contents
    search_and_print_matches(query, &contents);
}

// Function to search and print lines that contain the query
fn search_and_print_matches(query: &str, contents: &str) {
    for line in contents.lines() {
        if line.contains(query) {
            println!("{}", line);
        }
    }
}
