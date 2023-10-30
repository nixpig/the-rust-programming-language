use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let max_guesses = 10;
    let max_range = 100;

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=max_range);
    let mut counter = 0;

    loop {
        let mut guess = String::new();
        println!("Guesses used: {counter}");
        if counter == max_guesses - 1 {
            println!("This is your last guess!!")
        }

        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        counter += 1;

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small 😥"),
            Ordering::Greater => println!("Too high 😢"),
            Ordering::Equal => {
                println!("You win! 🥳");
                break;
            }
        }

        if counter == max_guesses {
            println!("You've used all of your guesses.");
            println!("The correct answer was: {secret_number}");
            println!("Better luck next time!");

            break;
        }
    }
}
