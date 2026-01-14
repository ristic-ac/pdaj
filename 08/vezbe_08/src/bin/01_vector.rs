fn main() {
    // Creating a new empty vector
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Creating a vector with initial values using the vec! macro
    let v2 = vec![1, 2, 3, 4, 5];

    // Accessing an element using indexing
    let third_indexing: &i32 = &v2[2];
    println!("The third element using indexing is {}", third_indexing);

    // Accessing an element using the get method
    match v2.get(2) {
        Some(third_get) => println!("The third element using get method is {}", third_get),
        None => println!("There is no third element."),
    }

    // Example of attempting to access out-of-bounds index
    println!("\nAttempting to access an out-of-bounds element:");
    // This will panic and terminate the program
    // let out_of_bounds = &v2[100];
    // println!("Out-of-bounds element using indexing: {}", out_of_bounds);

    // Safe access method using get that returns None if out-of-bounds
    match v2.get(100) {
        Some(value) => println!("Out-of-bounds element using get method: {}", value),
        None => println!("Using get method, there is no element at index 100."),
    }
}
