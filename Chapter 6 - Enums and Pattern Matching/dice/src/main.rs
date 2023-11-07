fn main() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => roll_again(),
    }

    let defined = Some(23);
    if let Some(d) = defined {
        println!("defined: {d}");
    }
}

fn add_fancy_hat() {
    println!("Put on a fancy hat!");
}

fn remove_fancy_hat() {
    println!("Take off a fancy hat!");
}

fn roll_again() {
    println!("Roll again...");
}
