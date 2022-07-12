// use crate::apple::conf::ipadd;
use serde::{Deserialize, Serialize};
// use spin::{RwLock, RwLockWriteGuard};
use crate::apple::Result;
// use std::{net::SocketAddr, str::FromStr, sync::Arc};
use super::{ChannelS,Buf, Msg,RoomIP,NetIP};
use tokio::time::{sleep, Duration};
use crate::godot_print;
use std::collections::HashMap;
use chashmap::CHashMap;



lazy_static! {
    static ref CACHE: CHashMap<NetIP, Sign> = Default::default();
}

pub struct Cursor;

impl Cursor {
    pub fn _get(key:NetIP)->Option<Sign>{
        let sign = CACHE.get(&key)?;
        Some(sign.to_owned())
    }

    pub fn exists(key:&NetIP)-> bool{
        CACHE.contains_key(key)
    }

    pub fn find()->IpMap{
        let data = CACHE.clone();
        IpMap{data}
    }
    pub fn replace_one(data:Data)->Option<Sign>{
        CACHE.insert(data.ipa,data.sign)
    }
    pub fn _delete_one(key:NetIP)->Option<Sign>{
        CACHE.remove(&key)
    }
}


/**
 * 
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Sign {
    Ready,
    Next(u16),
    Rigth(u16),
    Leave(u16),
    Finish,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data{
    pub ipa:NetIP,
    pub sign:Sign,
}

impl Data {
    pub fn new(ipa: NetIP, sign: Sign) -> Self { Self { ipa, sign } }
    
    pub fn ready(ipa:NetIP)->Self{
        Self { ipa, sign:Sign::Ready } 
    }
    pub fn next(&self)->Option<Self>{

        match self.sign {
            Sign::Ready => {
                Some(Self{ipa:self.ipa.clone(), sign:Sign::Next(self.ipa.port.clone())})
            }
            Sign::Next(port)=>{
                if port > 1024 && port < 49151 {
                    let port2;
                    if port == self.ipa.port{
                        port2 = self.ipa.port + 1;
                    }else
                    if port > self.ipa.port{
                        let offset = port - self.ipa.port;
                        port2 = self.ipa.port - (offset + 1);
                    }else{
                        let offset = self.ipa.port - port;
                        port2 = self.ipa.port + offset;
                    }
                    Some(Self{ipa:self.ipa.clone(), sign:Sign::Next(port2)})
                }else{
                    Some(self.finish())
                }
            }
            _=>None
        }
    }
    pub fn rigth(&self)->Option<Self>{
        match self.sign {
            Sign::Ready =>{
                Some(Self{ipa:self.ipa.clone(), sign:Sign::Rigth(self.ipa.port)})
            }
            Sign::Next(port) =>{
                Some(Self{ipa:self.ipa.clone(), sign:Sign::Rigth(port)})
            }
            _=>None  
        }
    }
    pub fn _leave(&self)->Option<Self>{
        if let Sign::Rigth(port) = self.sign{
            Some(Self{ipa:self.ipa.clone(), sign:Sign::Leave(port)})
        }else{None}
    }
    pub fn finish(&self)->Self{
        Self{ipa:self.ipa.clone(), sign:Sign::Finish}
    }

    
}




/** 
 * 网络映射
 */
#[derive(Debug,Clone)]
pub struct IpMap {
    pub data: CHashMap<NetIP,Sign>
}
impl IpMap {
    pub fn _new() -> Self {
        let data = CHashMap::new();
        Self {data}
    }

    /**
     * IP探测
     */
    pub async fn start()-> Result<()> {
        for (ipa,sign) in Cursor::find().data {
            let data = Data::new(ipa,sign);
            Self::ask(data).await?;
        }

        sleep(Duration::from_millis(1)).await;
        Ok(())
    }

    async fn ask_msg(data:Data)->Option<Msg>{
        let mut dict = HashMap::new();
        
        // from消息
        

        if let Some(ipa) = RoomIP::get_player().myself{
            let ipa = NetIP::new(ipa.ip,ipa.port);

            dict.insert("from",Data::new(ipa,Sign::Ready));

            // to消息
            dict.insert("to",data.clone());

            match data.sign {
                Sign::Ready=>{
                    if let Some(data) = data.next(){
                        Cursor::replace_one(data);
                    }
                }
                Sign::Next(port)=>{
                    // msg组装
                    let type1: String = "P2P-ASK".to_owned();
                    let mut msg = Msg::new(data.ipa.ip, port, type1.clone());
                    if let Ok(()) = msg.set_object(dict){
                        return Some(msg)
                    }
                }
                _=>{}
            };
            
        }
        None
    }


    async fn ask(data:Data)->Result<()>{
        if let Some(msg) = Self::ask_msg(data.clone()).await{
            let buf = msg.to_buf()?;
            godot_print!("Rust->本机发送P2P-ASK：{:?}",msg);
            ChannelS::set().send_async(buf).await?;

            // godot_print!("Rust->暂放发送...");
            // sleep(Duration::from_millis(1)).await;

            if let Some(data2) = data.next(){
                Cursor::replace_one(data2);
            }
            
        }

        Ok(())
    }

    /**
    * 收到数据并回复
    * msg 接收到的消息
    */ 
    pub async fn accept(msg:Msg)->Result<Msg>{
        let map_object:HashMap<&str,Data>  = msg.get_object()?;
        let a = map_object.get("from").unwrap();
        let b = a.rigth().unwrap();
        Cursor::replace_one(b);


        let c = map_object.get("to").unwrap();
        let mut c = c.clone();
        c.ipa = RoomIP::get_myself().unwrap();

        let d = c.rigth().unwrap();
        let mut msg = Msg::new(msg.ip, msg.port, "P2P-RSP".to_owned());
        msg.set_object(d)?;
        let buf = msg.to_buf()?;
        godot_print!("Rust->本机发送P2P-RSP：{:?}",msg);
        ChannelS::set().send_async(buf).await?;

        Ok(msg)
    }

    pub fn rsp(msg:Msg)->Result<()>{
        let data:Data  = msg.get_object()?;
        godot_print!("Rust->本机收到P2P-RSP DATA：{:?}",data);
        Cursor::replace_one(data);
        
        Ok(())
    }

    // /**
    // * 回复确认已收到
    // */ 
    // pub fn check(value:IpMap)->Result<()>{
    //     let ip = value.ipa.ip.clone();
    //     let port = value.port;
    //     let mut msg = Msg::new(ip, port, "P2P-CHK".to_owned());
    //     msg.set_object(value)?;
    //     let buf = msg.to_buf()?;
    //     ChannelS::set().send(buf)?;
    //     Ok(())
    // }


    // /**
    // * 收到的第三次握手
    // */ 
    // pub fn check2(msg:Msg)->Result<()>{
    //     let mut map_list =  IpMapList::new();
    //     let map:IpMap  = msg.get_object()?;

    //     for i in Cursor::find().ip_list {
    //         if i.ipa == map.ipa{
    //             let b = i.rigth();
    //             map_list.ip_list.push(b)

    //         }else{
    //             map_list.ip_list.push(i.clone())
    //         }
    //     }
    //     Cursor::save(map_list);
    //     Ok(())
    // }

}
