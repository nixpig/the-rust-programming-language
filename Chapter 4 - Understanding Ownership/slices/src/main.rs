fn main() {
    let multi_word_string = String::from("Lorem ipsum dolar sit amet.");
    let single_word_string = String::from("Supercalifragilisticexpealidocious");

    let multi_first = my_first_word(&multi_word_string);
    println!("The first word in 'multi word string' is: {multi_first}");

    let single_first = my_first_word(&single_word_string);
    println!("The first word in 'singe word string' is: {single_first}");

    let book_multi_first = book_first_word(&multi_word_string);
    println!("The first word in 'multi word string' is: {book_multi_first}");

    println!("---------------");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// Note: I got a waaayy different solution than the book :D
fn my_first_word(s: &str) -> &str {
    let words = match s.split_once(" ") {
        Some(v) => v,
        None => {
            return s;
        }
    };

    return words.0;
}

fn book_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}
