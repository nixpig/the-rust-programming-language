use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();

        println!("num: {}", num);

        *num = 23;
    }

    println!("m: {:?}", m);

    println!("----------");

    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut c = counter_clone.lock().unwrap();
            *c += 1;

            println!("{i}: {c}");
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("counter: {}", counter.lock().unwrap());
}
