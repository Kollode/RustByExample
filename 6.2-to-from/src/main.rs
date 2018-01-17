use std::string::ToString;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
struct Circle {
    radius: i32,
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

impl FromStr for Circle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Circle {
            radius: s.parse::<i32>()?,
        })
    }
}

/* impl FromStr for Circle {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(n) => Ok(Circle { radius: n }),
            Err(n) => Err(n),
        }
    }
} */

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!{"Sum: {:?}", sum};

    let circle: Circle = "20".parse().unwrap();
    println!{"Sum: {:?}", circle};

    let circle: Circle = String::from("20").parse().unwrap();
    println!{"Sum: {:?}", circle};
}
