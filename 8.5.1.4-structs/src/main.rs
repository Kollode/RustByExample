fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    struct Foo2 {
        x: u32,
        y: u32,
        z: u32,
    }

    // destructure members of the struct
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;

    println!("a = {}, b = {},  y = {} ", a, b, y);

    // you can destructure structs and rename the variables,
    // the order is not important

    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // and you can also ignore some variables:
    let Foo { y, .. } = foo;
    println!("y = {}", y);

    let foo2 = Foo2 { x: 7, y: 3, z: 2 };

    // Will ignore everything but y
    let Foo2 { y, .. } = foo2;
    println!("y = {}", y);

    // Ignore just a single variable, also possible with ref
    let Foo2 { y, x: _, ref z } = foo2;
    println!("y = {}, z = {}, isRef = {}", y, z, z == &foo2.z);

    // this will give an error: pattern does not mention field `x`
    // let Foo { y } = foo;
}
