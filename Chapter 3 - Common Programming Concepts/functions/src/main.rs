fn main() {
    another_function(5);

    print_labeled_measurement(23, 'h');

    println!("The result of five() is: {}", five());

    let five_plus_one = plus_one(5);
    println!("5 + 1 = {}", five_plus_one);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
