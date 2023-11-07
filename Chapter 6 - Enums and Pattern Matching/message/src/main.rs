#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: u32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let m1 = Message::Write(String::from("this is a write message"));
    let m2 = Message::Quit;
    let m3 = Message::Move { x: 23, y: 69 };
    let m4 = Message::ChangeColor(255, 120, 0);

    m1.call();
    m2.call();
    m3.call();
    m4.call();
}
