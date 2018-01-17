use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl From<f64> for Number {
    fn from(item: f64) -> Self {
        Number { value: item as i32 }
    }
}

impl From<String> for Number {
    fn from(item: String) -> Self {
        //match item.parse() {
        // With 'turbofish' ::<> to help compiler with type inference
        match item.parse::<i32>() {
            Ok(n) => Number { value: n },
            Err(n) => {
                println!("{}", n);
                Number { value: 0 }
            }
        }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
    println!("My number is {:?}", Number::from(10.50));
    println!("My number is {:?}", Number::from(String::from("20")));
    println!("My number is {:?}", Number::from(String::from("NaN")));

    let number: Number = 20.into();
    println!("My number is {:?}", number);

    let number: Number = String::from("10").into();
    println!("My number is {:?}", number);
}
