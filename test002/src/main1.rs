extern crate lazy_static;
use tokio::net::UdpSocket;
use std::net::SocketAddr;
use std::collections::HashMap;
mod apple;
use apple::conf::ipadd;
use serde::{Deserialize, Serialize};
use md5;


#[tokio::main]
 async fn main() {
    let sock = UdpSocket::bind(ipadd::Conf::local_server().parse::<SocketAddr>().unwrap()).await.unwrap();


    let mut map = HashMap::new();
    map.insert("type".to_owned(), "IP-ASK".to_owned());
        
    
    let send_buf = serde_json::to_vec(&map).unwrap();
    let addr = ipadd::Conf::remote_server();
    let len = sock.send_to(&send_buf, addr).await.unwrap();
    println!("{:?} bytes sent", len);

    let mut recv_buf = [0; 1024];

    let (len, addr) = sock.recv_from(&mut recv_buf).await.unwrap();
    println!("{:?} bytes received from {:?}", len, addr);
    
    let a = &recv_buf[..len];
    let digest = md5::compute(a);
    println!("digest is {:?}",digest);

    let obj:HashMap<String,String> = serde_json::from_slice(a).unwrap();
    println!("obj is {:?}",obj);

    match obj.get("object") {
        Some(rst)=>{
            
            println!("object is {:?}",rst);
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

