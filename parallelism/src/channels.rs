// Message passing with channels example
use std::sync::mpsc;
use std::thread;

//Channels for message passing between threads
pub fn main() {
    let (tx, rx) = mpsc::channel();
    for i in 0..5 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(format!("Message from thread {}", i)).unwrap();
        });
    }
    for _ in 0..5 {
        let msg = rx.recv().unwrap();
        println!("Received: {}", msg);
    }
}

