// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
#[derive(Debug, PartialEq)]
enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or like structures.
    Click { x: i64, y: i64 },

    PlainText,
    ApplicationJson,

    // Is not possible
    // Test = "This is a Value"
}

// c like enum
enum Test {
    IntValue = 7,
    Value = 8,
}

impl WebEvent {
    pub fn from_str(s: &str) -> Option<WebEvent> {
        match s {
            "text/plain" => Some(WebEvent::PlainText),
            "application/json" => Some(WebEvent::ApplicationJson),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            &WebEvent::PlainText => "text/plain",
            &WebEvent::ApplicationJson => "application/json",
            _ => "No String",
        }
    }
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
        _ => (),
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    println!("{:?}", WebEvent::KeyPress('x'));
    // This is only possible if the enum is implementing the PartialEq trait
    println!("Equals: {}", WebEvent::PageLoad == WebEvent::PageLoad);

    // Enums can not be mapped directly to values, they need some extra magic
    println!("{}", WebEvent::ApplicationJson.as_str());
    println!("{:?}", WebEvent::from_str("text/plain"));
    println!("{:?}", WebEvent::from_str("Unknown"));

    // enums can contain int values and then be cast to i32/i64
    println!("{}", Test::IntValue as i64);
}
