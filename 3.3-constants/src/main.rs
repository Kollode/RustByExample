// Globals are declared outside all other scopes.
static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

// This is not possible in global scope
// let test = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const` or ``static`.
    //THRESHOLD = 5;
    //LANGUAGE = "Test";
}
