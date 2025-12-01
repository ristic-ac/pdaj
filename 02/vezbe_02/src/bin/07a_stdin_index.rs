use std::io;

fn main() {
    let array1 = [10, 20, 30, 40, 50];
    println!("Array array1: {:?}", array1);

    println!("{}", String::from("Enter index [0-4], if not valid, rust will panic:"));
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let index: usize = input.trim().parse::<usize>().expect("Not a valid number");
    let value_indexed = array1[index];
    println!("Value at index {} is {}", index, value_indexed);
}
