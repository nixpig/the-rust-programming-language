use std::io;

fn main() {
    println!("Enter a temperature in Fahrenheit.");

    let temp: f64 = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("should be a temperature");

        break match input.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Invalid. Try again.");
                continue;
            }
        };
    };

    println!("{}F is {}C", temp, (temp - 32.0) * (5.0 / 9.0));
}
