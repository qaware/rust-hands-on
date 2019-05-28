use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use rand::Rng;

fn single_producer_channel() {
    // Creating a new channel returns transmitter and receiver
    let (tx, rx) = mpsc::channel();

    // Move tx into the thread
    thread::spawn(move || {
        // Send Hi from this thread
        // Sending may fail if the receiving end has been dropped
        let s = String::from("Hi");
        tx.send(s).expect("Sending failed");
        
        // s has been moved into the channel and cannot be used any more
        // println!("Sent: {}", s);
    });

    // Recive and print one item
    // Recv will block until there is an item on the channel
    println!("Got: {}", rx.recv().unwrap());
}

fn multi_producer_channel() {
    // Creating a new channel returns transmitter and receiver
    let (tx, rx) = mpsc::channel();
    // Clone the transmitter for a second producer
    // Must happen before move
    let tx1 = mpsc::Sender::clone(&tx);

    // First producer
    thread::spawn(move || {
        for _ in 0..5 {
            tx.send(String::from("Hi")).expect("Sending failed");
            thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(20, 80)));
        }
    });

    // Second producer
    thread::spawn(move || {
        for _ in 0..5 {
            tx1.send(String::from("Ho")).expect("Sending failed");
            thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(20, 80)));
        }
    });

    // Receive until all transmitters have been closed
    while let Ok(s) = rx.recv() {
        println!("{}", s);
    }
}

fn main() {
    single_producer_channel();

    multi_producer_channel();
}
