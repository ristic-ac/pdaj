fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");

    // This ability, to have multiple unmutable references and mutable references in the same scope, without overlapping use is called Non-lexical lifetimes.
    let r3 = &mut s;
    println!("{r3}");
}
