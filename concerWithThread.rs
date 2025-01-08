use std::thread;

fn main() {
    // Create a new thread
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread says: {}", i);
        }
    });

    // Main thread work
    for i in 1..5 {
        println!("Main says: {}", i);
    }

    // Wait for the spawned thread to finish
    handle.join().unwrap();
    println!("Main thread finished.");
}
