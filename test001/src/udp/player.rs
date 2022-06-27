use crate::apple::conf::ipadd;
use serde::{Deserialize, Serialize};
use spin::RwLock;
use crate::apple::Result;
use std::{net::SocketAddr, str::FromStr, sync::Arc};
use super::{ChannelS, Msg};
use std::collections::HashMap;

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

impl NetIP {
    pub fn new(ip: String, port: u16) -> Self { Self { ip, port } }
}

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
        Self { ip_list } 
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

    pub fn join(key:String)-> Result<Msg>{
        let url = ipadd::URL::remote_server();
        let ipa = SocketAddr::from_str(&url)?;
        let mut msg = Msg::new(ipa.ip().to_string(), ipa.port(), "ROOM-ASK".to_owned());
        let mut map = HashMap::new();
        map.insert("ROOM".to_owned(), key);
        msg.set_object(map)?;
        let buf = msg.to_buf()?;
        ChannelS::set().send(buf)?;
        Ok(msg)
    }
}
