mod pin_unpin;

use tokio::time::{sleep, Duration};
use tokio::join;
use pin_unpin::run;

/*
   future ~ Promises in JavaScript
   It's necessary to implement the Future trait
   In general, you should not poll a future again after it has returned Poll::Ready.
   If you do, the future is well within its rights to panic. A future that is safe to poll after it
   has returned Ready is sometimes referred to as a fused future.
 */

async fn task_one() {
    println!("Task one started");
    sleep(Duration::from_secs(2)).await;
    println!("Task one finished");
}

async fn task_two() {
    println!("Task two started");
    sleep(Duration::from_secs(1)).await;
    println!("Task two finished");
}

#[tokio::main]
async fn main() {
    join!(task_one(), task_two());
    println!("Both tasks completed");
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    run().await;
}



