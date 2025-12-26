use std::thread;

fn immutable_reference() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let list_ref = &list;

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure ref: {list_ref:?}");
    println!("After calling closure: {list:?}");
}

fn mutable_reference() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    // let list_ref = &list;

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    // println!("After calling closure once: {list_ref:?}");
    println!("After calling closure: {list:?}");
}

fn move_ownership() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

fn main(){
    immutable_reference();
    mutable_reference();
    move_ownership();
}