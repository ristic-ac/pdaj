// Example struct for destructuring patterns
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

// Enums for more complex pattern-matching
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
    Hello { id: i32 },
}

fn main() {
    // 1. Matching Literals
    let x = 1;
    match x {
        1 => println!("Matched one!"),
        2 | 3 => println!("Matched two or three!"),
        _ => println!("Matched anything else!"),
    }

    // let y = 2;
    // match y {
    //     1 | 2 => println!("Matched one or two"),
    //     2 => println!("Matched two specifically"), // This arm is unreachable
    //     _ => println!("Matched anything else"),
    // }
    // Rust compiler will produce an error: "unreachable pattern" for the second `2` arm.

    // 2. Matching Ranges
    let y = 5;
    match y {
        1..=5 => println!("Matched range from one to five!"),
        _ => println!("Matched something outside the range!"),
    }

    // 2.b Matching Ranges chars
    let ch = 'c';
    match ch {
        'a'..'z' => println!("Alphabet"), // This will cause an error because '..' is not inclusive.
        _ => println!("Non-alphabet character"),
    }
    // Rust only allows inclusive ranges with `..=`.

    // 3. Destructuring Structs
    let origin = Point { x: 0, y: 0, z: 0 };
    match &origin {
        Point { x: 0, y: 0, z: 0 } => println!("Origin!"), // Ignoring other fields with `..`
        Point { x, .. } => println!("x is {x}"), // Ignoring other fields with `..`
    }
    let Point { x: a, y: b, z:c } = origin;
    println!("a: {a}, b: {b}, c: {c}");

    // match origin {
    //     Point { x, y, .., z } => println!("Invalid usage of .. in the middle"), 
    // }
    // Rust only allows `..` to appear once and only at the end of patterns.

    // 4. Destructuring Enums
    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!("Color RGB: {r}, {g}, {b}"),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!("Color HSV: {h}, {s}, {v}"),
        Message::Move { x, y } => println!("Move to {x}, {y}"),
        _ => println!("Some other message"),
    }

    // let msg = Message::Quit;
    // match msg {
    //     Message::ChangeColor(_) => println!("Change color message"),
    //     Message::Move { x, y } => println!("Move message with x = {x}, y = {y}"),
    // }
    // Use `_ => ...` to cover remaining variants or explicitly match each one.

    // 5. Ignoring Values with `_`
    foo(3, 4);

    // 6. Ignoring Parts of Values with Nested `_`
    let new_setting_value = Some(10);
    let mut setting_value = Some(5);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite existing value"),
        _ => setting_value = new_setting_value,
    }
    println!("setting is now {:?}", setting_value);

    // 7. Using .. in Complex Structures
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => println!("First: {first}, Last: {last}"),
    }

    // let tuple = (2, 4, 6, 8);
    // match tuple {
    //     (.., middle, ..) => println!("This is ambiguous"), // ERROR: `..` can only be used once in tuples
    // }
    // `..` can only be used once per tuple pattern

    // 8. Extra Conditions with Match Guards
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("Even number: {x}"),
        Some(x) => println!("Odd number: {x}"),
        None => println!("No number"),
    }

    // 9. @ Bindings for Pattern-Matching
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => println!("Found id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => println!("Found id in another range"),
        Message::Hello { id } => println!("Some other id: {id}"),
        _ => {}
    }
}

// A function to demonstrate `_` in function parameters
fn foo(_: i32, y: i32) {
    println!("Function only uses y: {y}");
}
