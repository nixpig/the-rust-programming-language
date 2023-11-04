fn main() {
    let s1 = String::from("Hello");
    let l = calculate_length(&s1); // pass a _reference_ to s1 ('borrow' s1)
    println!("{s1} is {l} characters long.");

    // -----------------------------

    let mut s2 = String::from("Hello");
    change_str(&mut s2);
    println!("{s2}");

    // -----------------------------
}

// takes a _reference_ to a String; it's 'borrowed'
fn calculate_length(s: &String) -> usize {
    return s.len();
    // ^ s goes out of scope here, but the underlying _referenced_ data does not get dropped
}

// takes a _mutable reference_
fn change_str(s: &mut String) {
    s.push_str(", world!");
}
