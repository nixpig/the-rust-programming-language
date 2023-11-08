fn main() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    let mut v2 = vec![4, 5, 6];

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);

    println!("---");

    v1.append(&mut v2);

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);

    println!("---");

    let i2 = v1[2];
    println!("i2: {:?}", i2);

    let i9: Option<&i32> = v1.get(9);
    match i9 {
        Some(v) => println!("Value at position 9 is: {v}"),
        None => println!("Position 9 is not in the vector."),
    };

    println!("---");

    for i in &v1 {
        print!("{i}");
    }
    println!();

    for i in &mut v1 {
        *i *= 2;
    }

    println!("{:?}", v1);

    println!("---");

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let spreadsheet_row = vec![
        SpreadsheetCell::Int(23),
        SpreadsheetCell::Float(69.42),
        SpreadsheetCell::Text(String::from("foobar")),
    ];

    println!("{:?}", spreadsheet_row);
}
