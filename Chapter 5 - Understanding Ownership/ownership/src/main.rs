fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{s}");

    // x bound to 5
    let x = 5;
    // value of x (5) copied and bound to y
    let y = x;
    // integer has a known, fixed size; both 5 values are pushed onto the stack
    println!("x = {x}, y = {y}");

    // s1 has ptr, len, cap on stack; ptr points to memory on heap
    let s1 = String::from("Hello");
    // s2 has a copy of ptr, len, cap; ptr points to _same_ memory on heap
    let s2 = s1;

    // s1 is not longer available, since it is _moved_ into s2
    println!("{s2}, world!");

    // clone copies the heap data as well, which may be expensive!
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    // string literals (like integers) are stack allocated, so this is valid
    let h1 = "foo";
    let h2 = h1;
    println!("h1 = {h1}, h2 = {h2}");

    // ----------------------------------

    let s = String::from("Hello");
    take_ownership(s);
    // ^ takes ownership of heap allocated data; s is no longer valid after this point

    let n = 5;
    makes_copy(n); // makes a copy of stack data; n can still be accessed after
    println!("{n}");

    // ----------------------------------
    let s1 = gives_ownership();
    let s2 = String::from("world");
    let s3 = takes_and_gives_back(s2);
    println!("{s1}, {s3}!");

    // ----------------------------------
    let s1 = String::from("foobar");
    let (s, l) = calculate_length(s1);
    println!("{s} is {l} characters long.");
}

fn take_ownership(x: String) {
    println!("{x}")
}

fn makes_copy(x: i32) {
    println!("{x}");
}

fn gives_ownership() -> String {
    let s = String::from("Hello");

    return s;
}

fn takes_and_gives_back(s: String) -> String {
    return s;
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    return (s, length);
}
