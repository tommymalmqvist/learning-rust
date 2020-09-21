use std::fmt;

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "width: {}, height: {}", self.width, self.height)
    }
}

fn main() {
    let r1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    let r2: Rectangle = Rectangle {
        width: 10,
        height: 40,
    };
    let r3: Rectangle = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2: {}", r1.can_hold(&r2));
    println!("Can rect2 hold rect3: {}", r2.can_hold(&r3));
}
