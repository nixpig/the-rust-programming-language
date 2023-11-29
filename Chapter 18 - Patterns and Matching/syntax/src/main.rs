fn main() {
    let x = 2;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("not one, two or three!"),
    }

    match x {
        1 | 2 => println!("one or two"),
        _ => println!("something else"),
    }

    let mut y = vec![1, 2, 3];

    while let Some(n) = y.pop() {
        println!("{}", n);
    }

    let z = 23;

    match z {
        1..=10 => println!("one through ten"),
        11..=20 => println!("eleven through twenty"),
        21..=30 => println!("twenty-one through thirty"),
        _ => println!("something else"),
    }

    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point {
        x: 23,
        y: 42,
        z: 69,
    };

    match origin {
        Point { y, .. } => println!("y is: {y}"),
    }

    let numbers = (1, 3, 5, 13, 23, 42, 69);

    match numbers {
        (first, .., last) => println!("first: {first} | last: {last}"),
    }

    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("the number {x} is even"),
        Some(x) => println!("the number {x} is odd"),
        None => (),
    }

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 23 };

    match msg {
        Message::Hello { id: id @ 1..=10 } => println!("message with id ({id}) in range 1-10"),
        Message::Hello { id: id @ 11..=30 } => println!("message with id ({id}) in range 11-30"),
        _ => println!("message not in range 11-30"),
    }
}
