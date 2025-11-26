use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    // Array demonstrating Copy and Clone
    let mut array1: [i32; 3] = [1, 2, 3];
    let array2 = array1; // Copy occurs here, because i32 implements Copy (stored on stack)
    let array3 = array1.clone(); // Clone occurs here

    println!("Array 1: {:?}", array1);
    println!("Array 2 (Copy of Array 1): {:?}", array2);
    println!("Array 3 (Clone of Array 1): {:?}", array3);

    let array4: [i32; 3] = [2; 3];
    let array5: [i32; 3] = [1, 2, 3];

    println!("Array 4 == Array 5: {}", array4 == array5); // Eq
    println!("Array 4 > Array 5: {}", array4 > array5); // Ord

    match array4.cmp(&array5) {
        Ordering::Less => println!("Array 4 is less than Array 5"),
        Ordering::Equal => println!("Array 4 is equal to Array 5"),
        Ordering::Greater => println!("Array 4 is greater than Array 5"),
    }

    let array8: [f64; 3] = [1.0, 2.0, 3.0];
    let array9: [f64; 3] = [1.0, 2.0, 4.0];

    // Resize array8
    // let array8: [f64; 3] = [1.0, 2.0, 3.0, 4.0];

    println!("Array 8 == Array 9: {}", array8 == array9); // PartialEq
    println!("Array 8 > Array 9: {}", array8 > array9); // PartialOrd

    match array8.partial_cmp(&array9) {
        Some(Ordering::Less) => println!("Array 8 is less than Array 9"),
        Some(Ordering::Equal) => println!("Array 8 is equal to Array 9"),
        Some(Ordering::Greater) => println!("Array 8 is greater than Array 9"),
        None => println!("Array 8 and Array 9 are not comparable"),
    }

    // Demonstrating Hash
    let mut hasher = DefaultHasher::new();
    array1.hash(&mut hasher);
    let hash_value = hasher.finish();
    println!("Hash of Array 1: {}", hash_value);

    // Demonstrating AsRef and AsMut
    // Used predominantly for converting to references of other types
    let array6: [i32; 3] = [10, 20, 30];
    let slice_ref: &[i32] = array6.as_ref(); // AsRef
    println!("Array 6 as slice (AsRef): {:?}", slice_ref);

    let mut array7 = [5, 6, 7];
    let slice_mut: &mut [i32] = array7.as_mut(); // AsMut
    println!("Array 7 as mutable slice (AsMut): {:?}", slice_mut);
    slice_mut[0] = 50;
    println!("Array 7 as mutable slice (AsMut): {:?}", slice_mut);

    // Demonstrating Borrow and BorrowMut
    // Used predominantly for looking up values in collections (like HashMap) by converting to references (String to &str)
    let borrowed_slice: &[i32] = array6.borrow(); // Borrow
    println!("Borrowed slice (Borrow): {:?}", borrowed_slice);

    let borrowed_mut_slice: &mut [i32] = array7.borrow_mut(); // BorrowMut
    println!(
        "Borrowed mutable slice (BorrowMut): {:?}",
        borrowed_mut_slice
    );
    borrowed_mut_slice[0] = 100;
    println!(
        "Borrowed mutable slice (BorrowMut): {:?}",
        borrowed_mut_slice
    );

    // Demonstrating IntoIterator (consuming the array and iterating over it)
    println!("Iterating over Array 1 using IntoIterator:");
    for elem in array1 {
        println!("{}", elem);
    }

    // Iterating over a reference to an array (which uses &IntoIterator)
    println!("Iterating over Array 1 by reference:");
    for elem in &array1 {
        println!("{}", elem);
    }

    // Iterating over a mutable reference to an array (which uses &mut IntoIterator)
    println!("Iterating over Array 1 by mutable reference:");
    for elem in &mut array1 {
        println!("{}", elem);
    }

    // Debug trait is demonstrated through all the prints
    println!("Array 1: {:?}", array1);
    println!("Array 2 (Copy of Array 1): {:?}", array2);
    println!("Array 3 (Clone of Array 1): {:?}", array3);
    println!("Array 4: {:?}", array4);
}
