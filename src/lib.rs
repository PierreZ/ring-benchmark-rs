#![feature(async_await, await_macro, futures_api)]

use futures::channel::mpsc;
use futures::channel::mpsc::{Receiver, Sender};
use futures::Poll;
use futures::prelude::*;

struct Actor {
    is_first: bool,
    has_started: bool,
    should_loop: bool,
    init_value: i64,
    tx: Sender<i64>,
    rx: Receiver<i64>,
}

impl Actor {
    fn new(is_first: bool, init_value: i64, tx: Sender<i64>, rx: Receiver<i64>) -> Actor {
        Actor {
            is_first,
            has_started: false,
            should_loop: true,
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
            await!(self.tx.send(self.init_value + 1));
        }

        while self.should_loop {
            let nbr = await!(self.rx.next());
            match nbr {
                Some(mut nbr) => {
                    if self.is_first {
                        nbr -= 1;
                        println!("{}", nbr);
                        if nbr == 0 {
                            println!("stopping loop");
                            self.should_loop = false;
                        }
                    }
                    await!(self.tx.send(nbr));
                }

                // None represents that the tx has been shut, we need to propagate this
                None => {
                    println!("stopping loop");
                    self.should_loop = false;
                }
            }
        }
        println!("closing");
        await!(self.tx.close());
    }
}


async fn ring(nbr_processes: i64, nrb_iteration: i64) {
    let (first_tx, first_rx) = mpsc::channel(1);
    let (second_tx, second_rx) = mpsc::channel(1);

    let first = Actor::new(true, nrb_iteration, second_tx, first_rx);
    let second = Actor::new(false, nrb_iteration, first_tx, second_rx);

    runtime::spawn(first.handle());
    await!(second.handle());
}

#[cfg(test)]
mod tests {
    use crate::ring;

    #[runtime::test]
    async fn test_ring() {
        await!(ring(1, 10000));
    }
}
