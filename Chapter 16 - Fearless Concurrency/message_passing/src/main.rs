use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    let tx2 = tx.clone();

    let msg_parts = vec!["Hello", "from", "the", "thread"];

    let handle_tx1 = thread::spawn(move || {
        for m in msg_parts {
            tx1.send(m).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let handle_tx2 = thread::spawn(move || {
        for i in 1..=3 {
            tx2.send("foo").unwrap();
        }

        handle_tx1.join().unwrap();
    });

    let recv = rx.recv().unwrap();
    println!("recv: {recv}");

    let try_recv = rx.try_recv().unwrap();
    println!("try_recv: {try_recv}");

    for received in &rx {
        println!("Iterator: {received}");
    }

    handle_tx2.join().unwrap();
}
