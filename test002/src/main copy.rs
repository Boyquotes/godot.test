#[macro_use]
extern crate lazy_static;
use tokio::{net::UdpSocket};
use std::{io, net::SocketAddr, sync::Arc};
use flume::{unbounded, Receiver, Sender};


lazy_static! {
    // static ref S_CMSG:(Sender<cmsg::CMsg>, Receiver<cmsg::CMsg>) = unbounded();
}



#[tokio::main]
 async fn main() {
    let sock = UdpSocket::bind("0.0.0.0:8080".parse::<SocketAddr>().unwrap()).await.unwrap();
    let r = Arc::new(sock);
    let s = r.clone();
    let  (tx, rx) = unbounded();

    tokio::spawn(async move {
        
        let a = rx.recv_async().await.unwrap();
        println!("{:?}",a);

    });

    let mut buf = [0; 1024];
    loop {
        let (len, addr) = r.recv_from(&mut buf).await.unwrap();
        println!("{:?} bytes received from {:?}", len, addr);
        tx.send_async((buf[..len].to_vec(), addr)).await.unwrap();
    }
}