fn main() {
    let pair = (0, -2);
    // Will match with _ because this pattern is not available in the match
    // let pair = (1, -2);

    let pair = (10, -2);

    println!("Tell me about {:?}", pair);
    // Match can be used to destructure a tuple
    match pair {
        // Destructure the second
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        // This seems not possible todo inside a tuple
        // (5 | 10, y) => println!("First is `5` or `10` and `y` is `{:?}`", y),
        _ => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }
}
