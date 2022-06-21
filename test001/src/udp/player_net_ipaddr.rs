use spin::RwLock;
use crate::apple::conf::ipadd;
use serde::{Deserialize, Serialize};
use super::{Msg,ChannelS,Buf};
use std::{net::SocketAddr, sync::Arc, str::FromStr};
// use crate::apple::Result;
// use crate::godot_print;

lazy_static! {
    static ref IP_LIST:Arc<RwLock<Vec<String>>> = {
        let ip_list = Vec::new();
        Arc::new(RwLock::new(ip_list))
    };

}


/**
 * 玩家公网IP地址
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerNetIP;
impl PlayerNetIP {
    pub fn get()->Vec<String>{
        IP_LIST.read().to_vec()
    }

    pub fn set(ipa_list: Vec<String>){
        let mut ipal = IP_LIST.write();
        *ipal = ipa_list;
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RoomIP{
    key:u16,
    ip:String,
    port:u16
}

impl RoomIP {
    pub fn new(key: u16, ip: String, port: u16) -> Self { Self { key, ip, port } }

    pub fn join(&self) {
        let url = ipadd::URL::remote_server();
        let url = format!("http://{}{}",url,"/public/room/");

        let client = reqwest::blocking::Client::new();
        let res = client
            .put(url)
            .json(&self)
            .send()
            .unwrap();
        // godot_print!("{:?}", res.status());
        // godot_print!("{:?}", res.text());
    }
}
