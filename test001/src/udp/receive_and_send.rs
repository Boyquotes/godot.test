use tokio::net::UdpSocket;
use std::{net::SocketAddr, sync::Arc, str::FromStr};
use crate::apple::Result;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use flume::{unbounded, Receiver, Sender};
use crate::apple::conf::ipadd::URL;


lazy_static! {
    static ref SEND_CHANNEL:(Sender<Buf>,Receiver<Buf>) = unbounded();
    static ref RECV_CHANNEL:(Sender<Buf>,Receiver<Buf>) = unbounded();
}


#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Types {
    IP,
    ROOM,
    STATS,
    ACTION,
    NULL
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Msg {
    ip:String,
    port:u16,
    data: HashMap<String,String>
}
impl Msg {
    pub fn new(ip: String, port: u16, type1:String) -> Self { 
        let mut map = HashMap::new();
        map.insert("type".to_owned(),type1);
        Self{ip,port,data:map} 
    }
    pub fn set_object<T:Serialize>(&mut self,obj:T) -> Result<()> {
        let object = serde_json::to_string(&obj)?;
        self.data.insert("object".to_owned(),object);
        Ok(())
    }
    pub fn to_buf(&self) -> Result<Buf> {
        let bytes =  serde_json::to_vec(&self.data)?;
        Ok(Buf{ip:self.ip.clone(),port:self.port,bytes})
    }
    pub fn get_type(&self) -> Result<Types> {
        let unk = String::from("NULL");
        let type1 = self.data.get("type").unwrap_or(&unk);
        let b:Types = serde_json::from_str(type1)?;
        Ok(b)   
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
        Ok(Msg{ip:self.ip.clone(),port:self.port,data})
    }
    pub fn get_target(&self) -> String {
        format!("{}:{}",self.ip,self.port)
    }
}


/**
 * 发送通道
 */
pub struct ChannelS;
impl ChannelS {
    pub fn set()->Sender<Buf>{
        SEND_CHANNEL.0.clone()
    }
    pub fn get()->Receiver<Buf>{
        SEND_CHANNEL.1.clone()
    }
}
/**
 * 接收通道
 */
pub struct ChannelR;
impl ChannelR {
    pub fn set()->Sender<Buf>{
        RECV_CHANNEL.0.clone()
    }
    pub fn get()->Receiver<Buf>{
        RECV_CHANNEL.1.clone()
    }
}

/**UdpServer
 * UDP发送和接收服务
 * fn new() 创建socket任务（Task）
 * pub async fn start(&self)  启动服务
 * fn udp_sender(&self) 从SQueue队列通道接收数据，socket发送数据
 * fn udp_accept(&self) 从socket接收数据，转存数据RQueue队列通道
 */
pub struct Task{
    sock: UdpSocket,
}
impl Task {
    #[tokio::main]
    pub async fn start() {
        let task = Self::new().await.unwrap();
        let task2 = Arc::new(task);
        let task3 = task2.clone();
        tokio::spawn(async move {
            while let Err(err) = task2.udp_sender().await {
                println!("{:?}", err);
            }
        });
        loop {
            if let Err(err) = &task3.udp_accept().await {
                println!("{:?}", err);
            }
        }
    }
    async fn new() -> Result<Self> {
        let add = URL::local_server();
        let add = SocketAddr::from_str(&add)?;
        let sock = UdpSocket::bind(add).await?;
        Ok(Self{sock})
    }
    async fn udp_sender(&self) -> Result<usize> {
        let send_buf = ChannelS::get().recv_async().await?;
        Ok(self.sock.send_to(&send_buf.bytes, send_buf.get_target()).await?)
    }
    async fn udp_accept(&self) -> Result<()> {
        let mut recv_buf = [0; 1024];
        let (len, addr) = self.sock.recv_from(&mut recv_buf).await?;
        let ip = addr.ip().to_string();
        let port = addr.port();
        let bytes = recv_buf[..len].to_vec();
        let buf = Buf::new(ip,port,bytes);
        Ok(ChannelR::set().send_async(buf).await?)
    }
}
