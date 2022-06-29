use crate::apple::conf::ipadd;
use serde::{Deserialize, Serialize};
use spin::RwLock;
use crate::apple::Result;
use std::{net::SocketAddr, str::FromStr, sync::Arc};
use super::{ChannelS, Msg};


lazy_static! {
    static ref ROOM: Arc<RwLock<RoomIP>> = {
        let room = RoomIP::new();
        Arc::new(RwLock::new(room))
    };
}

/** 
 * 公网IP地址
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetIP {
    pub ip: String,
    pub port: u16,
}

// impl NetIP {
//     pub fn new(ip: String, port: u16) -> Self { Self { ip, port } }
// }

/**
 * 房间玩家IP列表
 */
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct RoomIP {
    pub ip_list: Vec<NetIP>
}

impl RoomIP {
    pub fn new() -> Self {
        let ip_list = Vec::new();
        Self {ip_list}
    }

    pub fn get_player() -> Vec<NetIP> {
        let room = ROOM.read();
        room.ip_list.clone()
    }

    pub fn get_player_to_string() -> Vec<String> {
        let mut listr = Vec::<String>::new();
        let room = ROOM.read();
        for i in room.ip_list.iter() {
            listr.push(format!("{}:{}", i.ip, i.port));
        }
        listr
    }

    pub fn put_player(&self) {
        let mut ipal = ROOM.write();
        *ipal = self.clone();
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
        let ip_list: Vec<NetIP> = msg.get_object()?;
        let room = Self{ip_list};
        room.put_player();
        let buf = msg.to_buf()?;
        Self::check(&msg.ip,msg.port,buf.get_md5())?;
        Ok(())
    }

    /**
    * 回复确认已收到
    */ 
    pub fn check(ip:&String,port:u16,md5:String)->Result<()>{
        let mut msg = Msg::new(ip.clone(), port, "ROOM-CHK".to_owned());
        msg.insert("MD5".to_owned(), md5);
        let buf = msg.to_buf()?;
        ChannelS::set().send(buf)?;
        Ok(())
    }
}
