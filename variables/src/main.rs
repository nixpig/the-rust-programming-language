fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    let y = 5;
    println!("The value of y is {}", y);

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in inner scope is {}", y);
    }

    println!("The value of y is {}", y);

    let spaces = "     ";
    let spaces = spaces.len(); // <- changes type of `spaces` from str to usize

    println!("Number of spaces is {}", spaces);

    let num: u32 = "42".parse().expect("Not a number!");
    println!("The value of num is {}", num);

    let tuple: (f64, char, bool) = (23.42, 'f', true);
    println!("First element in tuple: {}", tuple.0);
    println!("Second element in tuple: {}", tuple.1);
    println!("Third element in tuple: {}", tuple.2);
    let (x, y, z) = tuple;
    println!("Destructured tuple x: {}", x);
    println!("Destructured tuple y: {}", y);
    println!("Destructured tuple z: {}", z);

    let arr: [usize; 5] = [1, 2, 3, 4, 5];
    for el in arr {
        println!("el: {}", el);
    }

    println!("Third element: {}", arr[2]);

    let arr = [3; 5];
    for el in arr {
        println!("el: {}", el);
    }
}
