#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Function to demonstrate the error with FnOnce (moving a value out)
fn error_fnonce(list: &mut [Rectangle], value: String) {
    // let mut sort_operations_once = vec![];

    // Uncommenting this block will result in the error:
    // list.sort_by_key(|r| {
    //     sort_operations_once.push(value); // Move occurs here. Adding move to the closure makes it FnOnce, but the compiler is still signaling root error, which is moving value.
    //     r.width
    // });

    // This would cause an error, so we move to the fixes.
}

// Function to fix the issue by cloning the value
fn fix_with_clone(list: &mut [Rectangle], value: String) {
    let mut sort_operations_clone = vec![];

    list.sort_by_key(|r| {
        sort_operations_clone.push(value.clone()); // Clone to avoid moving
        r.width
    });

    println!("With Clone: {:?}", sort_operations_clone);
}

// Function to fix the issue by capturing a reference to the value
fn fix_with_reference(list: &mut [Rectangle], value: String) {
    let mut sort_operations_ref = vec![];

    let value_ref = value; // Move value into reference
    list.sort_by_key(|r| {
        sort_operations_ref.push(&value_ref); // Capture a reference to value
        r.width
    });

    println!("With Reference: {:?}", sort_operations_ref);
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let value = String::from("closure called");

    // Demonstrating the error with FnOnce (this is commented out to avoid compilation errors)
    // error_fnonce(&mut list, value.clone());

    // Fix 1: Using `.clone()` to avoid moving `value`
    fix_with_clone(&mut list, value.clone());

    // Fix 2: Using a reference to avoid moving `value`
    fix_with_reference(&mut list, value.clone());

    // Final sorted list after using references or cloning
    println!("Sorted List: {:?}", list);
}
