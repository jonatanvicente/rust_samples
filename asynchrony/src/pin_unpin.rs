use std::future::Future;
use std::pin::Pin;
use std::marker::Unpin;
use std::task::{Context, Poll};
use tokio::time::{sleep, Duration};
use tokio::join;

struct DelayedPrint {
    message: &'static str,
    delay: Duration,
    done: bool,
}

// Safe to move after being pinned
impl Unpin for DelayedPrint {}

impl Future for DelayedPrint {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if !self.done {
            self.done = true;
            let waker = cx.waker().clone();
            let delay = self.delay;
            let message = self.message;
            tokio::spawn(async move {
                sleep(delay).await;
                println!("{}", message);
                waker.wake();
            });
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

pub async fn run() {

    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Starting execution: running advanced async tasks with Pin and Unpin");

    let mut future1 = DelayedPrint {
        message: "Delayed task one finished",
        delay: Duration::from_secs(2),
        done: false,
    };
    let mut future2 = DelayedPrint {
        message: "Delayed task two finished",
        delay: Duration::from_secs(1),
        done: false,
    };

    //Pin: its location will not change, safe usage
    //Requires the type to be Unpin if it's moved or mutated after pinning
    let pinned1 = Pin::new(&mut future1);
    let pinned2 = Pin::new(&mut future2);

    join!(pinned1, pinned2);
    println!("Both advanced tasks completed");
}

