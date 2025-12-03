use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);

    let first = a[0];
    println!("first: {}", first);

    let second = a[1];
    println!("second: {}", second);

    // Input from stdin

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let index: usize = input.trim().parse().expect("Please type a number!");

    let element = a[index];
    println!("The value of element is: {}", element);
}
