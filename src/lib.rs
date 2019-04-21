#![feature(async_await, await_macro, futures_api)]

use std::fmt::Error;
use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::task::{Context, Poll, Waker};

struct Actor {
    is_first: bool,
    tx: Sender<i32>,
    rx: Receiver<i32>,
}

impl Future for Actor {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: & mut Context) -> Poll < Self::Output > {
        //TODO: Test if rx has message
        Poll::Pending
    }
}


fn hello(nbr_processes: i64, nrb_iteration: i64) {

    // for nbr_process in 0..nbr_processes {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let actor = Actor {
        is_first: false,
        tx,
        rx,
    };

    runtime::spawn(actor);
}

#[cfg(test)]
mod tests {
    #[runtime::test]
    async fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
