use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use flume::{unbounded, Receiver, Sender};
use crate::apple::Result;

lazy_static! {
    static ref SEND_CHANNEL:(Sender<Buf>,Receiver<Buf>) = unbounded();
    static ref RECV_CHANNEL:(Sender<Buf>,Receiver<Buf>) = unbounded();
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
    pub fn get_type(&self) -> Option<String> {
        let type1 = self.data.get("type")?.to_owned();
        Some(type1)
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


#[derive(Debug, Serialize, Deserialize,Clone)]
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