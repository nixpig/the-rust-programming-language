use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{self};

use std::fs::*; // use of `write` in function below

pub fn add_to_waitlist() {
    let mut map = HashMap::new();
    map.insert(23, 42);

    let mut set = HashSet::new();
    set.insert(69);
}

fn seat_at_table() {
    let res1: fmt::Result;
    let res2: io::Result<()>;

    write("foo.txt", "lorem");
}
