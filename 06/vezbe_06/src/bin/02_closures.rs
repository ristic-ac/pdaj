fn immutable_reference() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let list_ref = &list;

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows(); // This gets evaluated to Fn::call(&self, ...)
    println!("After calling closure ref: {list_ref:?}");
    println!("After calling closure: {list:?}");
}

fn mutable_reference() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    // let list_ref = &list;

    let mut borrows_mutably = || list.push(7);  // Closure mutably borrows `list`

    borrows_mutably(); // This gets evaluated to FnMut::call_mut(&mut self, ...)
    // println!("After calling closure once: {list_ref:?}");
    println!("After calling closure: {list:?}");
}

fn move_ownership() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // Move ownership of `list` into the closure
    let consumes_list = move || {
        println!("From closure: {list:?}");
        drop(list);
    };

    consumes_list(); // This gets evaluated to FnOnce::call_once(self, ...)
    // consumes_list(); // This will cause a compile-time error because `list` has been moved
}

fn main(){
    immutable_reference();
    mutable_reference();
    move_ownership();
}