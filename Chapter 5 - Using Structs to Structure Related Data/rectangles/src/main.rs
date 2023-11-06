#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    fn square(size: u32) -> Self {
        return Self {
            width: size,
            height: size,
        };
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square = Rectangle::square(23);

    println!("'rect1' can hold 'rect2'? {}", rect1.can_hold(&rect2));
    println!("'rect1' can hold 'rect3'? {}", rect1.can_hold(&rect3));

    println!(
        "Dimensions of 'sqaure' are: width - {}; height - {};",
        square.width, square.height
    )
}
