struct Verse {
    position: String,
    item: String,
}

fn main() {
    let positions = ["first", "second", "third", "fourth", "fifth"];
    let items = [
        "partridge in a pear tree",
        "turtle doves",
        "french hens",
        "calling birds",
        "five gold rings",
    ];

    for i in 1..=items.len() {
        println!("round: {i}");
        for day in 0..i {
            println!(
                "On the {} day of Christmas my true love gave to me {} {}",
                positions[day],
                day + 1,
                items[day]
            );
        }

        println!("---");
    }
}
