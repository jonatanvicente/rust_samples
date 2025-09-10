mod async_example;
mod rayon_iterators;
mod rayon_parallel;
mod channels;
mod shared_state;

use std::thread;
use std::time::Duration;


//Thread-based parallelism
#[tokio::main]
async fn main() {

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    println!("Thread-based parallelism example");
    let mut handles = vec![];
    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("Thread {} is running", i);
            thread::sleep(Duration::from_millis(500));
            println!("Thread {} is done", i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("All threads have finished.");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Async/await concurrency example");
    async_example::main().await;
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Channels for message passing between threads");
    channels::main();
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Data parallelism example using rayon");
    rayon_iterators::main();
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Parallel iterators with rayon");
    rayon_parallel::main();
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Shared state with Mutex and Arc");
    shared_state::main();


}
