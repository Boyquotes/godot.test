use crate::apple::conf::ipadd;
use serde::{Deserialize, Serialize};
use spin::RwLock;
use crate::apple::Result;
use std::{net::SocketAddr, str::FromStr, sync::Arc};
use super::{ChannelS, Msg,RoomIP,NetIP};
use tokio::time::{sleep, Duration};
use crate::godot_print;

lazy_static! {
    static ref MAP_LIST: Arc<RwLock<IpMapList>> = {
        Arc::new(RwLock::new(IpMapList::new()))
    };
}


pub struct Cursor;
impl Cursor {
    pub fn save(value:IpMapList) {
        let mut ipal = MAP_LIST.write();
        *ipal = value;
    }
    pub fn _clear(){
        let new_list =  IpMapList::new();
        Self::save(new_list);
    }
    pub fn find_one(ipa:NetIP)->Option<IpMap>{
        let map_list = MAP_LIST.read();
        for i in &map_list.ip_list{
            if i.ipa == ipa{
                return Some(i.clone())
            }
        }
        return None
    }
    pub fn find()->IpMapList{
        let ip_map_list = MAP_LIST.read();
        ip_map_list.clone()
    }
    pub fn _update_one(value:IpMap){
        let mut new_list =  IpMapList::new();

        for i in Self::find().ip_list {
            if i.ipa == value.ipa{
                new_list.ip_list.push(value.clone())
            }else{
                new_list.ip_list.push(i)
            }
        }
        Self::save(new_list);
    }
    pub fn replace_one(value:IpMap){
        let mut new_list =  IpMapList::new();
        match Self::find_one(value.ipa.clone()) {
            Some(_)=>{
                for i in Self::find().ip_list {
                    if i.ipa == value.ipa{
                        new_list.ip_list.push(value.clone())
                    }else{
                        new_list.ip_list.push(i)
                    }
                }
            }
            None=>{
                for i in Self::find().ip_list {
                    new_list.ip_list.push(i)
                }
                new_list.ip_list.push(value)
            }
            
        }
        Self::save(new_list);
    }
    pub fn delete_one(ipa:NetIP){
        let mut new_list =  IpMapList::new();
        for i in Self::find().ip_list {
            if i.ipa != ipa{
                new_list.ip_list.push(i)
            }
        }
        Self::save(new_list);
    }
}

/**
 * 
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Sign {
    Ready,
    Begin,
    Next,
    Finish,
    Rigth,
    Leave
}
/** 
 * 网络映射
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IpMap {
    pub ipa: NetIP,
    pub sign: Sign,
    pub port: u16,
}
impl IpMap {
    pub fn new(ipa:NetIP) -> Self {
        Self {ipa:ipa.clone(),sign:Sign::Ready,port:ipa.port}
    }

    pub fn begin(&self)->Self{
        let ipa = self.ipa.clone();
        let sign = Sign::Begin;
        let port = ipa.port;
        Self { ipa, sign, port }
    }
    pub fn next(&self)->Self{
        let sign = Sign::Next;
        let port:u16;
        if self.port > 1024 && self.port < 49151 {
            if self.port == self.ipa.port{
                port = self.ipa.port + 1;
            }else
            if self.port > self.ipa.port{
                let offset = self.port - self.ipa.port;
                port = self.ipa.port - (offset + 1);
            }else{
                let offset = self.ipa.port - self.port;
                port = self.ipa.port + offset;
            }
            return Self { ipa:self.ipa.clone(), sign, port };
        }else{
            return self.finish()
        }
    }
    pub fn finish(&self)->Self{
        let sign = Sign::Finish;
        return Self { ipa:self.ipa.clone(), sign, port:self.port }  
    }
    pub fn rigth(&self,port:u16)->Self{
        let sign = Sign::Rigth;
        return Self { ipa:self.ipa.clone(), sign, port } 
    }
    pub fn leave(&self)->Self{
        let sign = Sign::Leave;
        return Self { ipa:self.ipa.clone(), sign, port:self.port } 
    }
    pub fn to_msg(&self)->Result<Msg>{
        let type1: String = "P2P-ASK".to_owned();
        let ip = self.ipa.ip.clone();
        let port = self.port;
        let mut msg = Msg::new(ip, port, type1.clone());
        msg.set_object(self)?;
        Ok(msg)
    }



}


/**
 * 房间玩家IP列表
 */
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct IpMapList {
    pub ip_list: Vec<IpMap>
}

impl IpMapList {
    pub fn new() -> Self {
        let ip_list = Vec::new();
        Self {ip_list}
    }

    /**
     * IP探测
     */
    pub async fn start()-> Result<()> {
        let mut map_list =  IpMapList::new();
        for i in Cursor::find().ip_list {
            match i.sign{
                Sign::Begin =>{
                    let msg = i.to_msg()?;
                    godot_print!("发送p2p请求{:?}",msg);
                    let buf = msg.to_buf()?;
                    ChannelS::set().send_async(buf).await?;
                    map_list.ip_list.push(i.next());
                }
                
                Sign::Next =>{
                    let msg = i.to_msg()?;
                    godot_print!("发送p2p请求{:?}",msg);
                    let buf = msg.to_buf()?;
                    ChannelS::set().send_async(buf).await?;
                    map_list.ip_list.push(i.next());
                }          
                _=>{
                    map_list.ip_list.push(i.clone())
                }
            }
        }
        Cursor::save(map_list);
        sleep(Duration::from_secs(10)).await;
        Ok(())

    }

    // pub fn ask(){
    //         // 更新映射表
    //     for i in ip_list.clone(){
    //         let ip_map = IpMap::new(i);
    //         let ip_map = ip_map.begin();
    //         Cursor::replace_one(ip_map);
            
    //     }

    // }
    pub fn ask_(msg:Msg)->Result<()>{
        let mut map_list =  IpMapList::new();
     
        for i in Cursor::find().ip_list {
            if i.ipa.ip == msg.ip{
                map_list.ip_list.push(i.rigth(msg.port))

            }else{
                map_list.ip_list.push(i.clone())
            }
        }
        Cursor::save(map_list);
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
