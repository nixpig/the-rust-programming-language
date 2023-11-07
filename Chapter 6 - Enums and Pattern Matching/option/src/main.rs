fn main() {
    let some_number = Some(23);
    let some_str = Some("foobar");
    let absent_number: Option<i32> = None;

    println!("some number: {:?}", some_number);
    println!("some str: {:?}", some_str);
    println!("absent number: {:?}", absent_number);

    let another_number = Some(69);

    let added = some_number.unwrap() + another_number.unwrap();
    println!("added: {:?}", added);
}
