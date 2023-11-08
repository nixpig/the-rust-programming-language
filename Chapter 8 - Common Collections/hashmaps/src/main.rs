use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("blue", 23);
    scores.insert("red", 69);

    println!("{:?}", scores);

    let blue_score = scores.get("blue");
    println!("{:?}", blue_score);

    let green_score = scores.get("green");
    println!("{:?}", green_score);

    for (k, v) in &scores {
        println!("{k}: {v}");
    }

    println!("---");

    let has_blue = scores.entry("blue");
    println!("has_blue: {:?}", has_blue);

    let has_pink = scores.entry("pink");
    println!("has_pink: {:?}", has_pink);

    scores.entry("orange").or_insert(99);
    println!("{:?}", scores);

    let black = scores.entry("black").or_default();
    println!("{black}");

    let sentence =
        "the quick brown fox jumped over the lazy brown dog then ate the dog and jumped into bed";
    let mut word_count = HashMap::new();

    for word in sentence.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", word_count);
}
