use crate::apple::conf::ipadd;
use serde::{Deserialize, Serialize};
use spin::RwLock;
use crate::apple::Result;
use std::{net::SocketAddr, str::FromStr, sync::Arc};
use super::{ChannelS, Msg,RoomIP,NetIP};


lazy_static! {
    static ref MAP_LIST: Arc<RwLock<IpMapList>> = {
        Arc::new(RwLock::new(IpMapList::new()))
    };
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
    pub offset: i32,
}

impl IpMap {
    pub fn new(ipa:NetIP) -> Self { Self { ipa,sign:Sign::Ready, offset:0}}
    
    pub fn to_begin(&self)->Self{
        let ipa = self.ipa.clone();
        let sign = Sign::Begin;
        let offset = 1;
        Self { ipa, sign, offset }
    }

    pub fn to_next(&self)->Self{
        let port = self.ipa.port as i32 + self.offset;
        if port > 1024 && port < 49151 {
            let signum =  -1*self.offset.signum(); // 判断正负,计算得到新符号
            let offset = signum * (1+self.offset.abs()); //偏移量绝对值+1，冠以符号，得到新的偏移量
            let sign = Sign::Next;
            return Self { ipa:self.ipa.clone(), sign, offset };
        }else{
            return self.to_finish()
        }
    }

    pub fn to_finish(&self)->Self{
        let offset = self.offset;
        let sign = Sign::Finish;
        return Self { ipa:self.ipa.clone(), sign, offset }  
    }

    pub fn to_rigth(&self)->Self{
        let offset = self.offset;
        let sign = Sign::Rigth;
        return Self { ipa:self.ipa.clone(), sign, offset } 
    }

    pub fn to_leave(&self)->Self{
        let offset = self.offset;
        let sign = Sign::Leave;
        return Self { ipa:self.ipa.clone(), sign, offset } 
    }

    pub fn to_msg(&self)->Result<Msg>{
        let type1: String = "P2P-ASK".to_owned();
        let ip = self.ipa.ip.clone();
        let port = (self.ipa.port as i32 + self.offset) as u16;
        let ipa = NetIP::new(ip.clone(),port);
        let mut msg = Msg::new(ip, port, type1.clone());
        msg.set_object(ipa)?;
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

    pub fn save(&self) {
        let mut ipal = MAP_LIST.write();
        *ipal = self.clone();
    }

    pub fn clear(){
        let new_self =  IpMapList::new();
        Self::save(&new_self);
    }

    pub fn find_one(ipa:NetIP)->Option<IpMap>{
        let room = MAP_LIST.read();
        for i in &room.ip_list{
            if i.ipa == ipa{
                return Some(i.clone())
            }
        }
        return None
    }


    pub fn find()->Self{
        let ip_map_list = MAP_LIST.read();
        ip_map_list.clone()
    }


    pub fn update_one(value:IpMap){
        let mut new_self =  IpMapList::new();

        for i in Self::find().ip_list {
            if i.ipa == value.ipa{
                new_self.ip_list.push(value.clone())
            }else{
                new_self.ip_list.push(i)
            }
        }
        Self::save(&new_self);
    }

    pub fn replace_one(value:IpMap){
        let mut new_self =  IpMapList::new();
        match Self::find_one(value.ipa.clone()) {
            Some(_)=>{
                for i in Self::find().ip_list {
                    if i.ipa == value.ipa{
                        new_self.ip_list.push(value.clone())
                    }else{
                        new_self.ip_list.push(i)
                    }
                }
            }
            None=>{
                for i in Self::find().ip_list {
                    new_self.ip_list.push(i)
                }
                new_self.ip_list.push(value)
            }
            
        }
        Self::save(&new_self);
    }


    pub fn delete_one(&self,value:IpMap){
        let mut new_self =  IpMapList::new();
        for i in Self::find().ip_list {
            if i.ipa != value.ipa{
                new_self.ip_list.push(i)
            }
        }
        Self::save(&new_self);
    }
    


    /**
     * IP探测
     */
    pub async fn start()-> Result<()> {
        let mut map_list =  IpMapList::new();

        for i in Self::find().ip_list {
            match i.sign{
                Sign::Begin=>{
                    let a = i.to_next();
                    map_list.ip_list.push(a.clone());
                    
                    let msg = a.to_msg()?;
                    ChannelS::set().send_async(msg.to_buf()?).await?;
                }
                Sign::Next=>{
                    let a = i.to_next();
                    map_list.ip_list.push(a.clone());
                    let msg = a.to_msg()?;
                    ChannelS::set().send_async(msg.to_buf()?).await?;
                }
          
                _=>{
                    map_list.ip_list.push(i.clone())
                }
            }
        }


        map_list.save();
        // Self::save(&map_list);
        
        Ok(())

    }


    /**
    * 收到数据并回复
    * msg 接收到的消息
    */ 
    pub fn rsp(msg:Msg)->Result<()>{
        let mut p_map_list =  IpMapList::new();
        let map:IpMap  = msg.get_object()?;

        Self::check(msg.ip,msg.port,map.clone())?;

        for i in Self::find().ip_list {
            if i.ipa == map.ipa{
                let b = i.to_rigth();
                p_map_list.ip_list.push(b)

            }else{
                p_map_list.ip_list.push(i.clone())
            }
        }
        Self::save(&p_map_list);
        Ok(())
    }

    /**
    * 回复确认已收到
    */ 
    pub fn check(ip:String,port:u16,map:IpMap)->Result<()>{
        let mut msg = Msg::new(ip.clone(), port, "P2P-CHK".to_owned());
        msg.set_object(map)?;
        let buf = msg.to_buf()?;
        ChannelS::set().send(buf)?;
        Ok(())
    }


    /**
    * 收到的第三次握手
    */ 
    pub fn check2(msg:Msg)->Result<()>{
        let mut p_map_list =  IpMapList::new();
        let map:IpMap  = msg.get_object()?;

        for i in Self::find().ip_list {
            if i.ipa == map.ipa{
                let b = i.to_rigth();
                p_map_list.ip_list.push(b)

            }else{
                p_map_list.ip_list.push(i.clone())
            }
        }
        Self::save(&p_map_list);
        Ok(())
    }

}
