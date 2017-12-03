use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                // ? will return the value or if it was an error will return the error
                // The fm:tResult is a std::result::Result wich will return () as value 
                // or an Error. If we remove the ?, the compiler will fail, because we don't handle possible errors
                write!(f, ", ")?;

                // Could also be written with the try macro, but this is old
                // try!(write!(f, ", "));

                // Or we handle the error our self
                /* 
                    let write_result = write!(f, ", ");
                    if write_result.is_err() {
                        return write_result;
                    }
                */

            }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
