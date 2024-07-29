use std::sync::Arc;

use futures_intrusive::{
    buffer::FixedHeapBuf,
    channel::{GenericChannel, GenericOneshotChannel},
};

// oneshot
// #[tokio::main]
// async fn main() {
//     let ch: Arc<GenericOneshotChannel<parking_lot::RawMutex, i32>> =
//         Arc::new(GenericOneshotChannel::new());

//     let h = tokio::runtime::Handle::current();
//     let tx = ch.clone();
//     let rx = ch.clone();

//     let jh = h.spawn(async move {
//         let s = rx.receive().await.unwrap();
//         println!("{s}")
//     });
//     tx.send(3).unwrap();
//     jh.await.unwrap();
// }

#[tokio::main]
async fn main() {
    let ch: Arc<GenericChannel<parking_lot::RawMutex, i32, FixedHeapBuf<i32>>> =
        Arc::new(GenericChannel::new());

    let h = tokio::runtime::Handle::current();
    let tx = ch.clone();
    let rx = ch.clone();

    let jh = h.spawn(async move {
        let s = rx.receive().await.unwrap();
        println!("{s}")
    });
    tx.send(3).await.unwrap();
    jh.await.unwrap();
}
