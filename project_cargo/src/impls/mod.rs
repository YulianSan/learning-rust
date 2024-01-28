struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn set_width(&mut self, width: u32) {
        self.width = width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // without self is a static method :o
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn example1() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 10,
    };

    let square = Rectangle::square(60);

    rect1.set_width(50);

    println!(
        "The area of the rectangle is {} square pixels, width: {}, can hold {} {}",
        rect1.area(),
        rect1.width(),
        rect1.can_hold(&rect2),
        rect1.can_hold(&square),
    );
}
