use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a single thread
    let child_thread_1 = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Main thread
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Join, or else the child thread will be terminated when the main thread 
    // ends
    child_thread_1.join().unwrap();

    // Some string value. 
    let s = "🔥 tatütata 🚒";

    // The following won't compile as there's a lifetime mismatch. 
    // The thread might live longer than s
    // thread::spawn(|| {
    //     println!("{}", s)
    // }).join().unwrap();

    // Use move to force move values into the thread scope
    // Alternatively we could use a static lifetime for s
    thread::spawn(move || {
        println!("{}", s)
    }).join().unwrap();

    // Alternatively we could use a static lifetime for s
    static S: &str = "🍩 tatütata 🚓";
    thread::spawn(|| {
        println!("{}", S)
    }).join().unwrap();

}
