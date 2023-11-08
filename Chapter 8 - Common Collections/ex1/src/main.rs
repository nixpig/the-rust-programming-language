use std::collections::HashMap;

fn main() {
    let nums = [23, 13, 69, 7, 4, 42, 23, 13, 13];

    let mut vnums = nums.to_vec();

    vnums.sort();
    let mid = vnums.len() / 2;
    let median = vnums[mid];
    println!("median: {median}");

    let mut num_count = HashMap::new();

    for n in vnums {
        let mut count = num_count.entry(n).or_insert(0);
        *count += 1;
    }

    println!("{:?}", num_count);

    let mut counter = 0;
    let mut most_frequent: Option<i32> = None;

    for (k, v) in num_count {
        println!("{k}: {v}");
        if v > counter {
            counter = v;
            most_frequent = Some(k);
        }
    }

    match most_frequent {
        Some(v) => println!("Most frequent occurence is: {v}"),
        None => println!("There was no most frequently occuring number"),
    };
}
