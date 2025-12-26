// Variant 1: relies on lifetime elision in the return type, which can not be concluded here (at least for Rust <=2021). Error E0700
// In Rust 2018/2021, a return-position impl Trait does not automatically capture input lifetimes unless those lifetimes appear in the return type’s bounds.
// In Rust 2024, return-position impl Trait does automatically capture all in-scope generic parameters, including lifetimes. So in edition 2024, Variant 1 is treated as if it were Variant 3
fn make_a_cloner_elided(s_ref: &str) -> impl Fn() -> String {
    move || s_ref.to_string()
}

// Variant 2: explicit lifetime parameter on the function, explicitly carried to the return type.
fn make_a_cloner_explicit<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
    move || s_ref.to_string()
}

// Variant 3: explicit `'_` lifetime bound on the returned impl Trait.
// This forces the compiler to treat the returned closure as borrowing from `s_ref`.
fn make_a_cloner_underscore(s_ref: &str) -> impl Fn() -> String + '_ {
    move || s_ref.to_string()
}

// Helper: create the owned String we borrow from.
fn make_owned_string() -> String {
    String::from("Hello world")
}

// Helper: call a cloner and show the produced String.
fn run_cloner(label: &str, cloner: impl Fn() -> String) {
    let out = cloner();
    println!("{label}: {out}");
}

// Demo 1: elided lifetime variant.
fn demo_elided() {
    let s_own = make_owned_string();
    let cloner = make_a_cloner_elided(&s_own);
    // drop(s_own); // Uncommenting this will fail: `cloner` borrows from `s_own`.
    run_cloner("elided", cloner);
}

// Demo 2: explicit lifetime variant.
fn demo_explicit() {
    let s_own = make_owned_string();
    let cloner = make_a_cloner_explicit(&s_own);
    // drop(s_own); // Uncommenting this will fail for the same reason as above.
    run_cloner("explicit", cloner);
}

// Demo 3: `'_` lifetime variant.
fn demo_underscore() {
    let s_own = make_owned_string();
    let cloner = make_a_cloner_underscore(&s_own);
    // drop(s_own); // Uncommenting this will fail for the same reason as above.
    run_cloner("underscore", cloner);
}

fn main() {
    // Call all three demos to show they behave the same.
    demo_elided();
    demo_explicit();
    demo_underscore();
}
