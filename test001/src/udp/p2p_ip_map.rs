use crate::apple::conf::ipadd;
use serde::{Deserialize, Serialize};
use spin::{RwLock, RwLockWriteGuard};
use crate::apple::Result;
use std::{net::SocketAddr, str::FromStr, sync::Arc};
use super::{ChannelS, Msg,RoomIP,NetIP,PublicNetIP};
use tokio::time::{sleep, Duration};
use crate::godot_print;
use std::collections::HashMap;
use chashmap::CHashMap;



lazy_static! {
    static ref CACHE: CHashMap<NetIP, Sign> = Default::default();
}

pub struct Cursor;

impl Cursor {
    pub fn get(key:NetIP)->Option<Sign>{
        let sign = CACHE.get(&key)?;
        Some(sign.to_owned())
    }
    pub fn find()->IpMap{
        let data = CACHE.clone();
        // let mut data2:HashMap<NetIP, Sign>;
        // data.clone_into(&mut data2);
        IpMap{data}
    }
    pub fn replace_one(data:Data)->Option<Sign>{
        CACHE.insert(data.ipa,data.sign)
    }
    pub fn delete_one(key:NetIP)->Option<Sign>{
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
        if let Sign::Next(port) = self.sign{
            Some(Self{ipa:self.ipa.clone(), sign:Sign::Rigth(port)})
        }else{None}
    }
    pub fn leave(&self)->Option<Self>{
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
    pub fn new() -> Self {
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

        

        sleep(Duration::from_secs(10)).await;
        Ok(())
    }

    async fn ask_msg(data:Data)->Option<Msg>{
        let mut map = HashMap::new();

        if let Some(ipa) = PublicNetIP::read(){
            let ipa = NetIP::new(ipa.ip,ipa.port);
            let data2 = Data::new(ipa,Sign::Ready);
            map.insert("from",data2);
        }
        map.insert("to",data.clone());
        let type1: String = "P2P-ASK".to_owned();
        let ip = data.ipa.ip.clone();
        if let Sign::Next(port) = data.sign{
            let mut msg = Msg::new(ip, port, type1.clone());
            if let Ok(()) = msg.set_object(map){
                return Some(msg);
            }
        }
        None
    }


    async fn ask(data:Data)->Result<()>{
        if let Some(msg) = Self::ask_msg(data.clone()).await{
            let buf = msg.to_buf()?;
            godot_print!("Rust->本机发送P2P-ASK：{:?}",msg);
            ChannelS::set().send_async(buf).await?;
        }

        if let Some(data2) = data.next(){
            Cursor::replace_one(data2);
        }
        Ok(())
    }

    // /**
    // * 收到数据并回复
    // * msg 接收到的消息
    // */ 
    // pub fn rsp(msg:Msg)->Result<()>{
    //     let mut map_list =  IpMapList::new();
    //     let value:IpMap  = msg.get_object()?;
    //     Self::check(value.clone())?;
    //     for i in Cursor::find().ip_list {
    //         if i.ipa == value.ipa{
    //             let b = i.rigth();
    //             map_list.ip_list.push(b)

    //         }else{
    //             map_list.ip_list.push(i.clone())
    //         }
    //     }
    //     Cursor::save(map_list);
    //     Ok(())
    // }

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
