use remoc::{codec::JsonCodec, rsync::oneshot};

use crate::loop_channel;

#[tokio::test]
async fn simple() {
    crate::init();
    let ((mut a_tx, _), (_, mut b_rx)) = loop_channel::<oneshot::Receiver<i16, JsonCodec>>().await;

    println!("Sending remote oneshot channel receiver");
    let (tx, rx) = oneshot::channel();
    a_tx.send(rx).await.unwrap();
    println!("Receiving remote oneshot channel receiver");
    let rx = b_rx.recv().await.unwrap().unwrap();

    let i = 512;
    println!("Sending {}", i);
    tx.send(i).unwrap();

    let r = rx.await.unwrap();
    println!("Received {}", r);
    assert_eq!(i, r, "send/receive mismatch");
}

#[tokio::test]
async fn close() {
    crate::init();
    let ((mut a_tx, _), (_, mut b_rx)) = loop_channel::<oneshot::Sender<i16, JsonCodec>>().await;

    println!("Sending remote oneshot channel sender");
    let (tx, mut rx) = oneshot::channel();
    a_tx.send(tx).await.unwrap();
    println!("Receiving remote oneshot channel sender");
    let tx = b_rx.recv().await.unwrap().unwrap();

    assert!(!tx.is_closed());

    println!("Closing receiver");
    rx.close();

    println!("Waiting for close notification");
    tx.closed().await;

    match tx.send(0) {
        Ok(()) => panic!("send after close succeeded"),
        Err(err) if err.is_closed() => (),
        Err(err) => panic!("wrong error after close: {}", err),
    }
}
