fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The values are: x = {}, y = {}, z = {}", x, y, z);
    println!(
        "The values are: tup.0 = {}, tup.1 = {}, tup.2 = {}",
        tup.0, tup.1, tup.2
    );

    let mut tup: (i32, f64, u8) = (500, 6.4, 1);
    tup.0 = 600;
    println!("The updated value of tup.0 is: {}", tup.0);
}
