use crate_sample::add_one;
fn main() {
    let num: i32 = 10;
    let result: i32 = add_one(num);
    println!("The result is: {}", result);
    let result2: i32 = crate_sample::add_any(num, 20);
    println!("The second result is: {}", result2);
}