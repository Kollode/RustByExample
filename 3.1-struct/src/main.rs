#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

#[derive(Debug)]
struct PairStruct(Point, Point);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    let point1 = Point { x: 17.0, y: 20_f32 };

    let point2 = Point {
        x: 5_f32,
        y: 30_f32,
    };

    let pair_struct = PairStruct(point1, point2);
    println!("{:?}", pair_struct);

    let Point { x: my_x, y: my_y } = pair_struct.0;
    println!("x:{}, y:{}", my_x, my_y);

    let rectangle = Rectangle {
        p1: Point { x: my_y, y: my_x },
        p2: pair_struct.1,
    };

    println!("Rect: {:?}, Area: {}", rectangle, react_area(&rectangle));

    let squared = square(
        Point {
            x: 10_f32,
            y: 20_f32,
        },
        15_f32,
    );
    println!("Square: {:?}, Area: {}", squared, react_area(&squared));
}

fn react_area(rectangle: &Rectangle) -> f64 {
    let &Rectangle {
        p1: Point { x: p1x, y: p1y },
        p2: Point { x: p2x, y: p2y },
    } = rectangle;
    return ((p1x - p2x) * (p1y - p2y)) as f64;
}

fn square(point: Point, length: f32) -> Rectangle {
    let Point { x: p1x, y: p1y } = point;

    Rectangle {
        p1: point,
        p2: Point {
            x: p1x + length,
            y: p1y + length,
        },
    }
}
