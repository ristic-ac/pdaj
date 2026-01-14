// fn invalid_modification_during_iteration() {
//     let mut v = vec![1, 2, 3];
//     for i in &mut v {
//         v.push(*i); // This will cause an error: Cannot modify `v` during iteration
//     }
// }

fn valid_modification_with_indices() {
    let mut v = vec![1, 2];
    let length = v.len(); // length is 2
    for i in 0..length {
        v.push(v[i]); // Temporarily borrows v[i] immutably, then pushes a copy into v
    }
    println!("Duplicated vector: {:?}", v);
}

fn main() {
    // Uncommenting the function below will produce a compile-time error due to iterator safety
    // invalid_modification_during_iteration();

    valid_modification_with_indices();
}
