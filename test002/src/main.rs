#[macro_use]
extern crate lazy_static;
use tokio::{net::UdpSocket};
use std::{io, net::SocketAddr, sync::Arc};
use flume::{unbounded, Receiver, Sender};
use std::collections::HashMap;
mod apple;
use apple::conf::ipadd;
use serde::{Deserialize, Serialize};
lazy_static! {
    // static ref S_CMSG:(Sender<cmsg::CMsg>, Receiver<cmsg::CMsg>) = unbounded();
}



#[tokio::main]
 async fn main() {
    let sock = UdpSocket::bind(ipadd::Conf::local_server().parse::<SocketAddr>().unwrap()).await.unwrap();
    let field_name = String::from("type");
    let field_value = String::from("IP");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let buf = serde_json::to_vec(&map).unwrap();



    let addr = ipadd::Conf::remote_server();

    let len = sock.send_to(&buf, addr).await.unwrap();
    println!("{:?} bytes sent", len);

    let mut buf2 = [0; 1024];
    let (len, addr) = sock.recv_from(&mut buf2).await.unwrap();
    println!("{:?} bytes received from {:?}", len, addr);
    let obj:HashMap<String,String> = serde_json::from_slice(&buf2[..len]).unwrap();
    println!("obj is {:?}",obj);

    match obj.get("object") {
        Some(rst)=>{
            println!("object is {:?}",rst);
            let ip:IP = serde_json::from_str(rst).unwrap();
            println!("ip is {:?}",ip);
        }
        None =>{
            println!("空");
        }  
    };
    

}


#[derive(Debug, Serialize, Deserialize, Clone)]
struct IP {
    ip:String,
    port:	u16
}