use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    name: String,
    value: i32,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(name: &str, value: i32) -> Self {
        Self {
            name: name.to_string(),
            value,
            children: Vec::new(),
        }
    }
}

fn main() {
    // Root node is owned by `root_handle`.
    let root_handle: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node::new("root", 0)));

    // Child node will be owned by multiple handles.
    let child_handle: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node::new("child", 10)));

    // Root stores a clone of the child's Rc in its children list.
    {
        // Mutable borrow of root only lives in this scope.
        let mut root = root_handle.borrow_mut();
        root.children.push(Rc::clone(&child_handle));
    } // root borrow ends here

    // Another independent owner of the same child allocation.
    let observer_handle = Rc::clone(&child_handle);

    // Mutate child via observer_handle.
    {
        let mut child = observer_handle.borrow_mut();
        child.value += 5;
    } // child borrow ends here

    // Read child via root_handle (shows the same updated value).
    {
        let root = root_handle.borrow(); // immutable borrow of root
        let first_child = Rc::clone(&root.children[0]); // clone Rc; cheap
        drop(root); // end root borrow early (often useful)

        let child = first_child.borrow(); // immutable borrow of child
        println!("child via root = {} ({})", child.value, child.name); // 15 (child)
    }

    println!("child strong_count = {}", Rc::strong_count(&child_handle));
}
