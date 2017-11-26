
// With derive we add a automatic generated Debug function to the Person struct
// The 'a is a lifetime annotation. I'm not a 100% sure how they work yet.
// But because the str are references we need them here.
#[derive(Debug)]
struct Person<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: u8,
}

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // The primitives can be debugged
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // This will only work if the struct derives Debug
    println!("Now {:?} will print!", Structure(3));
    // With # we can do a 'pretty' print
    println!("Now {:#?} will print!", Deep(Structure(3)));

    // Struct with named attributes
    // Yay json notation
    let person = Person {
        first_name: "Patrick",
        last_name: "Kollodzik",
        age: 29,
    };

    println!("Hi, I'm {:#?}", person);
}
