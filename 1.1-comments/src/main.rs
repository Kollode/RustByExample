
/// This is a doc comment that will be used do describe the main function
/// in the generated doc. But only if we make the main method public.
/// The document can be generated with `cargo doc` and will be placed in the target directory
/// **Note:** I don't know if it is bad, if the main method is public.
pub fn main() {
    // This is a single line comment

    /*
        If we really want to, we can also create
        multi line comments.
    */

    // We even can create comment inside code.
    // And easily change the result of the calculation
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}
