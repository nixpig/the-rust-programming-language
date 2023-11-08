fn main() {
    let mut s = String::new();
    println!("Empty: {s}");

    let data = "Initial contents";

    s = data.to_string();
    println!("{s}");

    s = String::from(data);
    println!("{s}");

    s.push_str(" + more!");
    println!("{s}");

    let foo = String::from("foo");
    let bar = String::from("bar");

    let concat = foo + " " + &bar;
    println!("concat: {concat}");

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let s = format!("{tic}-{tac}-{toe}");
    println!("{s}");

    let tic_tac_toe = tic + " " + &tac + " " + &toe;
    println!("{tic_tac_toe}");

    let s = String::from("hello");
    for c in s.chars() {
        println!("{c}");
    }
}
