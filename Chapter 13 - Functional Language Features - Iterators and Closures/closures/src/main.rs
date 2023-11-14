use std::{thread, time::Duration};

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColour {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColour>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColour>) -> ShirtColour {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColour {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColour::Red => num_red += 1,
                ShirtColour::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColour::Red
        } else {
            ShirtColour::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColour::Blue, ShirtColour::Red, ShirtColour::Blue],
    };

    let user_pref1 = Some(ShirtColour::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let expensive_closure = |num: u64| -> u64 {
        println!("Counting down from {}", num);
        thread::sleep(Duration::from_secs(num));
        num
    };

    expensive_closure(2);

    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }

    let add_one_v2 = |x: u32| -> u32 { x + 1 };

    let add_one_v3 = |x| x + 1;

    let first = add_one_v1(1);
    let second = add_one_v2(first);
    let third = add_one_v3(second);

    println!("{}, {}, {}", first, second, third);

    let list = vec![1, 2, 3];
    println!("list before calling closure: {:?}", list);

    thread::spawn(move || {
        println!("list moved to new thread: {:?}", list);
    })
    .join()
    .unwrap();

    let mut rects = vec![
        Rectangle {
            width: 3,
            height: 9,
        },
        Rectangle {
            width: 1,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 3,
        },
    ];

    println!("{:?}", rects);

    rects.sort_by_key(|x| x.width);

    println!("{:?}", rects);
}
