use std::fmt::Display;

// Define a struct that holds a reference, which requires an explicit lifetime annotation
struct ImportantExcerpt<'a> {
    part: &'a str, // Lifetime 'a indicates that `ImportantExcerpt` cannot outlive the reference it holds
}

impl<'a> ImportantExcerpt<'a> {
    // Method with one reference parameter (`&self`), applying the third elision rule
    // Inferred lifetime: fn level<'a>(&'a self) -> &'a str
    fn level(&self) -> &str {
        self.part
    }

    // Method with multiple references, demonstrating the third elision rule
    // Inferred lifetimes:
    // fn announce_and_return_part<'a>(&'a self, announcement: &str) -> &'a str
    // `self`'s lifetime `'a` is assigned to the output because of the third rule.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Announcement: {}", announcement);
        self.part
    }

    // New method: stores a &'b str into self.part: &'a str
    // Constraint 'b: 'a is required so the assigned reference is valid for 'a.
    fn set_part_from<'b>(&'a mut self, new_part: &'b str)
    where
        'b: 'a,
    {
        self.part = new_part;
    }
}

// Function that returns the first word from a string
// Lifetime elision is applied here based on the first and second rules:
// Inferred lifetime: fn first_word<'a>(s: &'a str) -> &'a str
// Since there’s one input lifetime, it's assigned to the output lifetime.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Function to return the longer of two string slices
// Explicit lifetime annotation is required here as there are multiple inputs and no clear rule applies:
// Defined lifetime: fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
// Here, we explicitly declare that both inputs and the output share the same lifetime `'a`.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // Using the `first_word` function without explicit lifetime annotations
    let sentence = String::from("The quick brown fox");
    let word = first_word(&sentence); // `first_word` inferred as fn(&'a str) -> &'a str
    println!("First word: {}", word);

    // Creating an instance of `ImportantExcerpt`, which requires a lifetime annotation
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = ImportantExcerpt { part: first_sentence }; // `part` borrows from `novel`, so `'a` is linked to `novel`

    // Using a method with inferred lifetimes
    println!("Excerpt part: {}", excerpt.level()); // `level` inferred as fn(&'a self) -> &'a str

    // Using a method that returns a reference with lifetime elision
    println!("Announced excerpt: {}", excerpt.announce_and_return_part("Here's an excerpt")); 
    // `announce_and_return_part` inferred as fn(&'a self, &str) -> &'a str

    // Using `longest` with explicit lifetime annotations
    let string1 = String::from("long string");
    let string2 = "short";
    let result = longest(string1.as_str(), string2); // `longest` uses explicit lifetimes, returning the longest string with lifetime `'a`
    println!("The longest string is: {}", result);
}
