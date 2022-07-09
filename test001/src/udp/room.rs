use crate::apple::conf::ipadd;
use serde::{Deserialize, Serialize};
use spin::RwLock;
use crate::apple::Result;
use std::{net::SocketAddr, str::FromStr, sync::Arc};
use super::{ChannelS, Msg,Buf,Cursor,Data,Sign};
use super::godot_print;



lazy_static! {
    static ref ROOM: Arc<RwLock<RoomIP>> = {
        let room = RoomIP::new();
        Arc::new(RwLock::new(room))
    };


    static ref KEY: Arc<RwLock<Option<String>>> = {
        Arc::new(RwLock::new(None))
    };

}

/** 
 * 公网IP地址
 */
#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Clone)]
pub struct NetIP {
    pub ip: String,
    pub port: u16,
}

impl NetIP {
    pub fn new(ip: String, port: u16) -> Self { Self { ip, port } }
}


/**
 * 房间玩家IP列表
 */
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct RoomIP {
    pub myself: Option<NetIP>,
    pub ip_list: Vec<NetIP>
}

impl RoomIP {
    pub fn new() -> Self {
        let ip_list = Vec::new();
        Self {myself:None,ip_list}
    }

    pub fn get_myself()->Option<NetIP>{
        let room = ROOM.read();
        room.myself.clone()
    }

    pub fn get_player() -> RoomIP {
        let room = ROOM.read();
        room.clone()
    }

    pub fn save(&self) {
        let mut ipal = ROOM.write();
        *ipal = self.clone();
    }

    pub fn key_set(key:String){
        let mut rkey = KEY.write();
        *rkey = Some(key);
    }

    pub fn key_get()->Option<String>{
        let key = KEY.read();
        key.clone()
    }

    /** 
     * 加入房间请求
    */
    pub fn ask(key:String)-> Result<()>{
        let url = ipadd::URL::remote_server();
        let ipa = SocketAddr::from_str(&url)?;
        let mut msg = Msg::new(ipa.ip().to_string(), ipa.port(), "ROOM-ASK".to_owned());
        msg.insert("ROOM".to_owned(), key);
        let buf = msg.to_buf()?;
        ChannelS::set().send(buf)?;
        Ok(())
    }

    /**
    * 收到数据并回复
    * msg 接收到的消息
    */ 
    pub fn rsp(msg:Msg)->Result<()>{
        let room: RoomIP = msg.get_object()?;
        // 更新映射表
        for i in room.ip_list.clone(){
            let data = Data::ready(i);
            Cursor::replace_one(data);
        }
        godot_print!("收到房间信息{:?}",room);
        room.save();
        Ok(())
    }

    /**
    * 回复确认已收到
    */ 
    pub fn check(buf:Buf)->Result<()>{
        let mut msg = Msg::new(buf.ip.clone(), buf.port, "ROOM-CHK".to_owned());
        msg.insert("MD5".to_owned(), buf.get_md5());
        godot_print!("收到房间信息,回复确认{:?}",msg);
        let buf = msg.to_buf()?;
        ChannelS::set().send(buf)?;
        Ok(())
    }
}
