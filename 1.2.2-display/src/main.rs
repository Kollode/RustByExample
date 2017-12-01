
use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "(0:{}, 1:{})", self.0, self.1);
    }
}

#[derive(Debug)]
struct Point2D<T, Z> {
    x: T,
    y: Z,
}

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
}
