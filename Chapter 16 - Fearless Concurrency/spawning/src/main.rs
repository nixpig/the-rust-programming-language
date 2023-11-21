use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread.");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi number {i} from the main thread.");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    println!("-----");

    let v = vec![1, 2, 3, 4, 5];

    let handle = thread::spawn(move || {
        println!("v: {:?}", v);
    });

    handle.join().unwrap();
}
