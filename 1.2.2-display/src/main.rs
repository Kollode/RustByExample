// Is optional, but then std::fmt must be always used fmt::Formatter -> std::fmt::Formatter
// std is always loaded. std can be disabled in the compiler.
use std::fmt;

// create tuple struct, that gets the Debug trait
#[derive(Debug)]
struct MinMax(i64, i64);

// Now we implement the Display trait, the function head is specified in the documentation
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "(0:{}, 1:{})", self.0, self.1);
    }
}

// create a struct with two generics
#[derive(Debug)]
struct Point2D<T, Z> {
    x: T,
    y: Z,
}

// Because the struct uses generics we must implement the Display trait for every combination we want to use
// For example a Point<i32, i64> could not be display, because we don't implement the trait fr these generics
impl fmt::Display for Point2D<i64, i64> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "(x in i64:{}, y in i64:{})", self.x, self.y);
    }
}

impl fmt::Display for Point2D<i32, i32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "(x in i32:{}, y in i32:{})", self.x, self.y);
    }
}

fn main() {
    let minmax = MinMax(0, 14);
    println!("Display: {}", minmax);

    let i32NumberX: i32 = 3;
    let i32NumberY: i32 = 4;

    let point_i64: Point2D<i64, i64> = Point2D { x: 3, y: 4 };
    let point_i32 = Point2D {
        x: i32NumberX,
        y: i32NumberY,
    };

    println!("Display i64: {}", point_i64);
    println!("Display i32: {}", point_i32);

        // Error. Both `Debug` and `Display` were implemented but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // println!("What does Point2D look like in binary: {:b}?", point_i64);
}
