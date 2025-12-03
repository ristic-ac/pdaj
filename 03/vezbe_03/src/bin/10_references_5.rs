fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];

    // We borrow the reference to the third element of the vector.
    let num: &mut i32 = &mut v[2];
    // We dereference the reference to the third element of the vector, and we create immutable reference to it.
    let num2: &i32 = &*num;
    
    // *num dereferences the &mut i32 to i32.
    // &*num then takes an immutable reference to that i32.
    // This is called “reborrowing”: you create a shared (&i32) borrow from a mutable (&mut i32) borrow.
    // After this point, the compiler treats num as frozen for mutation while num2 is in use, but you can still read through num.

    // This in turn means that the num reference to the third element of the vector is immutable.
    // *num += 1;
    println!("{} {}", *num, *num2);
}