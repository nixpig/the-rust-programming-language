fn main() {
    let mut counter = 1;
    loop {
        println!("iteration: {}", counter);

        counter += 1;

        if counter > 3 {
            break;
        }
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count: {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("lift off!");

    let collection = ["a", "b", "c", "e", "f"];

    for item in collection {
        println!("item: {item}");
    }

    for number in (1..=3).rev() {
        println!("{number}");
    }
    println!("lift off!");
}
