#[derive(Debug)]
enum State {
    Alaska,
    Alabama,
    Arkansas,
    // etc...
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Option<State>),
}

impl Coin {
    fn print(&self) {
        return match self {
            Coin::Penny => println!("$0.01"),
            Coin::Nickel => println!("$0.05"),
            Coin::Dime => println!("$0.10"),
            Coin::Quarter(state) => {
                print!("$0.25");
                match state {
                    Some(v) => print!(" from {:?}", v),
                    None => (),
                }
                println!();
            }
        };
    }
}

fn main() {
    let p = Coin::Penny;
    let n = Coin::Nickel;
    let d = Coin::Dime;
    let q = Coin::Quarter(None);

    let s = Coin::Quarter(Some(State::Alaska));

    p.print();
    n.print();
    d.print();
    q.print();

    s.print();
}
