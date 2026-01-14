fn main() {
    let mut v = vec![1, 2, 3, 4, 5]; // A mutable vector is created.

    let first = &v[0]; // An immutable reference to the first element of `v` is created here.

    v.push(6); // This line attempts to modify the vector `v` by adding a new element.

    println!("The first element is: {first}");
}
