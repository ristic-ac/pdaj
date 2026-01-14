fn main() {
    // Using match expression with pattern matching
    let config_value = Some(10);
    match config_value {
        Some(v) if v > 5 => println!("Config value is greater than 5: {}", v),
        Some(v) => println!("Config value: {}", v),
        None => println!("No config value found"),
    }

    // Using if let with a complex condition
    let favorite_color: Option<&str> = Some("blue");
    let is_monday = false;
    let age: Result<u8, _> = "25".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}!", color);
    } else if is_monday {
        println!("It's Monday, let's use green!");
    } else if let Ok(age) = age {
        if age < 30 {
            println!("You're under 30, so we'll use yellow.");
        } else {
            println!("Over 30? We'll go with purple.");
        }
    } else {
        println!("No preferences, default to gray.");
    }

    // Using while let to loop while values are available
    let mut numbers = vec![10, 20, 30];
    while let Some(num) = numbers.pop() {
        println!("Popped number: {}", num);
    }

    // Using a for loop with pattern destructuring
    let letters = vec!['a', 'b', 'c'];
    for (index, letter) in letters.iter().enumerate() {
        println!("Letter {} at index {}", letter, index);
    }

    // Using let destructuring for tuples
    let (x, y, z) = (1, 2, 3);
    println!("Destructured tuple values: x={}, y={}, z={}", x, y, z);

    // Using function parameters with pattern matching
    let point = (5, -5);
    print_coordinates(&point);
}

// A function that takes a tuple and uses destructuring in its parameter
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
