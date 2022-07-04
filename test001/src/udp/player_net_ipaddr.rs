use crate::apple::conf::ipadd;
use serde::{Deserialize, Serialize};
use spin::RwLock;
use crate::apple::Result;
use std::{net::SocketAddr, str::FromStr, sync::Arc};
use super::{ChannelS, Msg,RoomIP,NetIP};


lazy_static! {
    static ref PNML: Arc<RwLock<PlayerNetMapList>> = {
        Arc::new(RwLock::new(PlayerNetMapList::new()))
    };

    // static ref P2PNET: (Sender<PlayerNetMap>, Receiver<PlayerNetMap>) = unbounded();
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
pub struct PlayerNetMap {
    pub ipa:NetIP,
    pub sign: Sign,
    pub offset: i32,
}

impl PlayerNetMap {
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
pub struct PlayerNetMapList {
    pub ip_list: Vec<PlayerNetMap>
}

impl PlayerNetMapList {
    pub fn new() -> Self {
        let ip_list = Vec::new();
        Self {ip_list}
    }

    pub fn get()->Self{
        let room = PNML.read();
        Self{ip_list:room.ip_list.clone()}
    }

    pub fn save(&self) {
        let mut ipal = PNML.write();
        *ipal = self.clone();
    }
    pub fn _del(&self,value:PlayerNetMap){
        let mut new_list =  PlayerNetMapList::new();
        for i in Self::get().ip_list {
            if i.ipa != value.ipa{
                new_list.ip_list.push(i)
            }
        }
        Self::save(&new_list);
        
    }
    pub fn insert(&self,value:PlayerNetMap){
        let mut new_list =  PlayerNetMapList::new();

        for i in Self::get().ip_list {
            if i.ipa == value.ipa{
                new_list.ip_list.push(value.clone())

            }else{
                new_list.ip_list.push(i)
            }
        }
        Self::save(&new_list);
    }


    /**
     * IP探测
     */
    pub async fn start()-> Result<()> {
        let mut p_map_list =  PlayerNetMapList::new();

        for i in Self::get().ip_list {
            match i.sign{
                Sign::Begin=>{
                    let a = i.to_next();
                    p_map_list.ip_list.push(a.clone());
                    
                    let msg = a.to_msg()?;
                    ChannelS::set().send_async(msg.to_buf()?).await?;
                }
                Sign::Next=>{
                    let a = i.to_next();
                    p_map_list.ip_list.push(a.clone());
                    let msg = a.to_msg()?;
                    ChannelS::set().send_async(msg.to_buf()?).await?;
                }
          
                _=>{
                    p_map_list.ip_list.push(i.clone())
                }
            }
        }

        Self::save(&p_map_list);
        
        Ok(())

    }


    /**
    * 收到数据并回复
    * msg 接收到的消息
    */ 
    pub fn rsp(msg:Msg)->Result<()>{
        let mut p_map_list =  PlayerNetMapList::new();
        let map:PlayerNetMap  = msg.get_object()?;

        Self::check(msg.ip,msg.port,map.clone())?;

        for i in Self::get().ip_list {
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
    pub fn check(ip:String,port:u16,map:PlayerNetMap)->Result<()>{
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
        let mut p_map_list =  PlayerNetMapList::new();
        let map:PlayerNetMap  = msg.get_object()?;

        for i in Self::get().ip_list {
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
