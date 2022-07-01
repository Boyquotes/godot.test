use crate::apple::conf::ipadd;
use serde::{Deserialize, Serialize};
use spin::RwLock;
use crate::apple::Result;
use std::{net::SocketAddr, str::FromStr, sync::Arc};
use super::{ChannelS, Msg,RoomIP};


lazy_static! {
    static ref PNML: Arc<RwLock<PlayerNetMapList>> = {
        Arc::new(RwLock::new(PlayerNetMapList::new()))
    };
}

/** 
 * 网络映射
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerNetMap {
    pub ipadd: String,
    pub port1: u16,
    pub port2: u16,
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

    pub fn get_list() -> Vec<PlayerNetMap> {
        let room = PNML.read();
        room.ip_list.clone()
    }

    pub fn save(&self) {
        let mut ipal = PNML.write();
        *ipal = self.clone();
    }


    /**
     * IP探测
     */
    pub fn probe(){

        // let type1: String = "ACTION-NEW".to_owned();

        // // 这里需要针对对称NAT做更改。IP端口需要通过试探得到映射端口
        // for i in RoomIP::get_player() {
        //     let mut msg = Msg::new(i.ip, i.port, type1.clone());
        //     msg.set_object(self)?;
        //     let buf = msg.to_buf()?;
        //     ChannelS::set().send(buf)?;
        // }

    }


}
