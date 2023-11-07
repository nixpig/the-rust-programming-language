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

    fn perimeter(&self) -> u32 {
        return self.width * 2 + self.height * 2;
    }
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        return Self { width, height };
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

    let rect = Rectangle::new(5, 10);
    println!("rect is: {:?}", rect);
    println!("perimeter of rect is: {}", rect.perimeter());

    println!("'rect1' can hold 'rect2'? {}", rect1.can_hold(&rect2));
    println!("'rect1' can hold 'rect3'? {}", rect1.can_hold(&rect3));

    println!(
        "Dimensions of 'sqaure' are: width - {}; height - {};",
        square.width, square.height
    )
}
