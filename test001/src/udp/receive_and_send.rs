use tokio::net::UdpSocket;
use std::{net::SocketAddr, sync::Arc};
use crate::apple::Result;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use flume::{unbounded, Receiver, Sender};
use std::thread;
lazy_static! {
    static ref SEND_CHANNEL:(Sender<Buf>,Receiver<Buf>) = unbounded();
    static ref RECV_CHANNEL:(Sender<Buf>,Receiver<Buf>) = unbounded();
}




pub fn start(){
    thread::spawn(move || {
        udp_server();
    });
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Msg {
    ip:String,
    port:u16,
    data: HashMap<String,String>
}

impl Msg {
    pub fn new(ip: String, port: u16, type1:String) -> Self { 
        let map = HashMap::new();
        map.insert("type".to_owned(),type1);
        Self{ip,port,data:map} 
    }

    pub fn set_object<T:Serialize>(&self,obj:T) -> Result<()> {
        let object = serde_json::to_string(&obj)?;
        self.data.insert("object".to_owned(),object);
        Ok(())
    }

    pub fn to_buf(&self) -> Result<Buf> {
        let bytes =  serde_json::to_vec(&self.data)?;
        Ok(Buf{ip:self.ip,port:self.port,bytes})
    }

    pub fn get_object<'a, T>(&'a self) -> Result<T> 
    where
        T: Deserialize<'a>,
    {
        let a = self.data.get("object").unwrap();
        let object = serde_json::from_str(a).unwrap();
        Ok(object)
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Buf {
    pub ip:String,
    pub port:u16,
    pub bytes: Vec<u8>,
}
impl Buf {

    pub fn new(ip:String,port:u16,bytes:Vec<u8>) -> Self {
        Self {ip,port,bytes}
    }

    pub fn to_msg(&self) -> Result<Msg> {
        let data:HashMap<String,String> = serde_json::from_slice(&self.bytes)?;
        Ok(Msg{ip:self.ip,port:self.port,data})
    }

    pub fn get_target(&self) -> String {
        format!("{}:{}",self.ip,self.port)
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecvMsg {
    ip:String,
    port:u16,
    data: Vec<u8>
}



/**
 * 发送通道
 */
pub struct SQueue;
impl SQueue {
    pub fn get_sender()->Sender<Buf>{
        SEND_CHANNEL.0.clone()
    }
    pub fn get_receiver()->Receiver<Buf>{
        SEND_CHANNEL.1.clone()
    }
}
/**
 * 接收通道
 */
pub struct RQueue;
impl RQueue {
    pub fn get_sender()->Sender<Buf>{
        RECV_CHANNEL.0.clone()
    }
    pub fn get_receiver()->Receiver<Buf>{
        RECV_CHANNEL.1.clone()
    }
}

#[tokio::main]
async fn udp_server() {
    let udps1 = match UdpServer::new().await {
        Ok(rst) => rst,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };
    let udps2 = Arc::new(udps1);
    let udps3 = udps2.clone();

    tokio::spawn(async move {
        while let Err(err) = udps3.udp_sender().await {
            println!("{:?}", err);
        }
    });

    loop {
        if let Err(err) = udps2.udp_accept().await {
            println!("{:?}", err);
        }
    }
}


/**UdpServer
 * UDP发送和接收服务
 * fn new() 创建 udp socket
 * fn udp_sender(&self) 从SQueue队列通道接收数据，socket发送数据
 * fn udp_accept(&self) 从socket接收数据，转存数据RQueue队列通道
 */
struct UdpServer {
    sock: UdpSocket,
}
impl UdpServer {

    async fn new() -> Result<Self> {
        let sock = UdpSocket::bind("0.0.0.0:8080".parse::<SocketAddr>()?).await?;
        Ok(Self {sock})
    }

    async fn udp_sender(&self) -> Result<usize> {
        let send_buf = SQueue::get_receiver().recv_async().await?;
        Ok(self.sock.send_to(&send_buf.bytes, send_buf.get_target()).await?)
    }

    async fn udp_accept(&self) -> Result<()> {
        let mut recv_buf = [0; 1024];
        let (len, addr) = self.sock.recv_from(&mut recv_buf).await?;
        let ip = addr.ip().to_string();
        let port = addr.port();
        let bytes = recv_buf[..len].to_vec();
        let buf = Buf::new(ip,port,bytes);
        Ok(RQueue::get_sender().send_async(buf).await?)
    }
}



