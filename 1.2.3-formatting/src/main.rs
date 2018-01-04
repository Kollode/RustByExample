use std::fmt::{self, Display, Formatter};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        return write!(f, "RGB ({}, {}, {})", self.red, self.green, self.blue);
    }
}

// Can find out if a # was part of the formatting with f.alternate
impl fmt::LowerHex for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let alternate_hex = if f.alternate() == true { "0x" } else { "" };
        return write!(
            f,
            "{}{:02x}{:02x}{:02x}",
            alternate_hex,
            self.red,
            self.green,
            self.blue
        );
    }
}

impl fmt::UpperHex for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let alternate_hex = if f.alternate() == true { "0x" } else { "" };
        return write!(
            f,
            "{}{:02X}{:02X}{:02X}",
            alternate_hex,
            self.red,
            self.green,
            self.blue
        );
    }
}

fn main() {
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ].iter()
    {
        println!("{}", *city);
    }
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ].iter()
    {
        // What does the star mean?
        println!("{color} {color:#x} {color:X}", color = *color);
    }
}
