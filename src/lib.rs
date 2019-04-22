#![feature(async_await, await_macro, futures_api)]

use futures::channel::mpsc;
use futures::channel::mpsc::{Receiver, Sender};
use futures::Poll;
use futures::prelude::*;

struct Actor {
    is_first: bool,
    has_started: bool,
    init_value: i64,
    tx: Sender<i64>,
    rx: Receiver<i64>,
}

impl Actor {
    fn new(is_first: bool, init_value: i64, tx: Sender<i64>, rx: Receiver<i64>) -> Actor {
        Actor {
            is_first,
            has_started: false,
            init_value,
            tx,
            rx,
        }
    }

    async fn handle(mut self) {

        // init first message
        if self.is_first && !self.has_started {
            self.has_started = true;
            println!("init");
            await!(self.tx.send(self.init_value));
        }

        let (nbr, _) = await!(self.rx.into_future());
        if let Some(mut nbr) = nbr {
            println!("received {}", nbr);
            if self.is_first  {
                nbr -= 1;
                if nbr < 0 {
                    println!("closing channel");
                    await!(self.tx.close());
                }
            }
            await!(self.tx.send(nbr));
        }
        println!("finished :(");
    }
}


async fn ring(nbr_processes: i64, nrb_iteration: i64) {
    let (first_tx, first_rx) = mpsc::channel(1);
    let (second_tx, second_rx) = mpsc::channel(1);

    let first = Actor::new(true, nrb_iteration, second_tx, first_rx);
    let second = Actor::new(false, nrb_iteration, first_tx, second_rx);

    runtime::spawn(first.handle());
    runtime::spawn(second.handle());
}

#[cfg(test)]
mod tests {
    use crate::ring;

    #[runtime::test]
    async fn test_ring() {
        await!(ring(1, 10));
    }
    // output:
    // init
    // received 10
    // received 10
    // finished :(
    // finished :(
    // test tests::test_ring ... ok
}
