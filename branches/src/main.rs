fn main() {
    let number = 69;

    if number < 5 {
        println!("condition is true")
    } else {
        println!("condition is false")
    }

    if number != 0 {
        println!("the number is not zero")
    }

    if number % 4 == 0 {
        println!("{} is divisible by 4", number)
    } else if number % 3 == 0 {
        println!("{} is divisible by 3", number)
    } else if number % 2 == 0 {
        println!("{} is divisible by 2", number)
    } else {
        println!("{} is not divisible by 4, 3 or 2", number)
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("conditional number is: {}", number)
}
