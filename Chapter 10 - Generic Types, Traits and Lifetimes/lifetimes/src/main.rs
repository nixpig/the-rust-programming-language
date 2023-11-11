use std::fmt::Display;

fn main() {
    let string1 = "abcd";
    let string2 = "xyz";
    let string3 = "jklmn";

    let result = longest(string1, string2, string3);

    println!("The longest string is: {result}");

    let lwa = longest_with_announcment(string1, string2, "News");

    println!("{lwa}");
}

fn longest<'a>(x: &'a str, y: &'a str, z: &'a str) -> &'a str {
    let mut long = x;

    if y.len() > long.len() {
        long = y;
    }

    if z.len() > long.len() {
        long = z;
    }

    long
}

fn longest_with_announcment<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}!", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
