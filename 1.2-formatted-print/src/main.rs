fn main() {
    // {} will replaced with the 31
    println!("{} days", 31);

    // We can do calculation inside the argument
    println!("The result of 2+2: {}", 2 + 2);

    // This is not possible, because we have more placeholder then values
    // println!("{} days and {} month", 31);

    // println will add a newline to the end of the printed string
    // with print we can display a string without newline
    print!("First part of the line");
    println!(" and the end.");

    // Or we add \n as a newline
    print!("First line\nSecond line\n");

    // The print will go to stdout and eprint to stderr
    eprintln!("This is important, because it failed");

    // We can also save the formated string in a variable
    //
    // Note: formattedString would lead to compiler warning,
    // because rust expects snake case names
    let formatted_string = format!("The result of 2+2: {}", 2 + 2);
    println!("FormattedString: {}", formatted_string);

    // We can use indices to insert a value more the once.
    // The index is based on the order of the arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // We can also make indices more speaking and give them names
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // A index with only numbers or starting with numbers is not valid
    // println!("{1111}", 1111 = "The not working index");

    // If we use a placeholder with an index that is not there
    // we can't compile.
    // println!("{firstName} {lastName}", firstName = "Patrick");

    // It will also not compile if we have more arguments then we use.
    // println!(
    //     "{firstName} {lastName}",
    //     firstName = "Patrick",
    //     lastName = "Kollodzik",
    //     nickName = "Kollode"
    // );

    // Special formatting can be specified after a `:`.
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1,
        2
    );

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number = 1, width = 6);

    // You can also do a left align, but wit spaces you will not see much of ad difference
    println!("{number:<width$}", number = 1, width = 6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number = 1, width = 6);

    // The version with left alignment
    println!("{number:0<width$}", number = 1, width = 6);

    // It is also possible to use chars for the padding.
    // I think it is better to write the 0 directly behind the
    // : so I don't get it mixed up with the 'width'.
    // For now I don't know if it makes a difference.
    println!("{number:B>width$}", number = 1, width = 6);
    println!("{number:B<width$}", number = 1, width = 6);

    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    // Create a structure which contains an `i32`. Name it `Structure`.
    // #[allow(dead_code)] will disable th compiler warning for unused code
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.

    // Print pi with only 3 decimal places
    // Everything after the : is for formatting, see: https://doc.rust-lang.org/std/fmt/#formatting-parameters
    let pi = 3.14159265359;
    println!("Google tells me Pi is {:.3}", pi);
}
