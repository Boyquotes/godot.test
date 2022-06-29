#[macro_use]
extern crate lazy_static;
use tokio::{net::UdpSocket};
use std::net::SocketAddr;
use std::collections::HashMap;
mod apple;
use apple::conf::ipadd;
use serde::{Deserialize, Serialize};
use json;
use md5;


#[tokio::main]
 async fn main() {
    let addr = ipadd::Conf::remote_server();
    let sock = UdpSocket::bind(ipadd::Conf::local_server().parse::<SocketAddr>().unwrap()).await.unwrap();
    
    let mut map = HashMap::new();
    map.insert("type".to_owned(), "ROOM-ASK".to_owned());
    map.insert("ROOM".to_owned(), "Test01".to_owned());
    
    let buf = serde_json::to_vec(&map).unwrap();
    
    let len = sock.send_to(&buf, &addr).await.unwrap();
    println!("发送请求{:?},长度{:?}.", map,len);

    let mut buf2 = [0; 1024];
    let (len2, addr2) = sock.recv_from(&mut buf2).await.unwrap();
    
    
    let a = &buf2[..len2];
    let digest = md5::compute(a);
    println!("buf2 digest is {:?}",digest);
    let obj2:HashMap<String,String> = serde_json::from_slice(a).unwrap();
    println!("接收回复数据{:?}，长度为{:?}", obj2, len2);

    match obj2.get("object") {
        Some(rst)=>{
            println!("object is {:?}",rst);

            // 确认接收
            let mut map3 = HashMap::new();
            map3.insert("type".to_owned(), "ROOM-CHK".to_owned());
            let md5 = format!("{:?}",digest);
            map3.insert("MD5".to_owned(), md5);
            let buf3 = serde_json::to_vec(&map3).unwrap();
            let len3 = sock.send_to(&buf3, addr2).await.unwrap();
            println!("发送确认{:?} 长度{:?}", map3,len3);

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


