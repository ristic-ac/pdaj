fn main() {
    let good = "123";
    let bad1 = "42a";
    let bad2 = "   456\n";  // contains whitespace/newline

    // Example 1: parse success
    let n: i32 = good.parse().unwrap();
    println!("Parsed '{}': {}", good, n);

    // Example 2: parse invalid string → Err
    match bad1.parse::<i32>() {
        Ok(v) => println!("Parsed '{}': {}", bad1, v),
        Err(e) => println!("Failed to parse '{}': {}", bad1, e),
    }

    // Example 3: parse with whitespace/newline → Err
    match bad2.parse::<i32>() {
        Ok(v) => println!("Parsed '{}': {}", bad2, v),
        Err(e) => println!("Failed to parse '{}': {}", bad2, e),
    }

}
