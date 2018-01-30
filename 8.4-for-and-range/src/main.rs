fn main() {
    let mut names = vec![1, 2, 3];

    println!("{:?}", names);

    // The name is just a reference, to change the value we need to use *name
    for name in names.iter_mut() {
        *name = *name + 2;
    }

    println!("{:?}", names);
}
