use spin::RwLock;
use crate::apple::conf::ipadd;
use serde::{Deserialize, Serialize};
use std::sync::Arc;


lazy_static! {
    static ref IP_LIST:Arc<RwLock<Vec<PlayerNetIP>>> = {
        let ip_list = Vec::new();
        Arc::new(RwLock::new(ip_list))
    };

}


/**
 * 玩家公网IP地址
 */

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct PlayerNetIP{
    pub ip:String,
    pub port:u16
}


impl PlayerNetIP {
    pub fn get_list()-> Vec<PlayerNetIP> {
        let ip_list = IP_LIST.read();
        ip_list.clone()
    }

    pub fn set_list(ipa_list: Vec<PlayerNetIP>){
        let mut ipal = IP_LIST.write();
        *ipal = ipa_list;
    }

    pub fn get_list_to_string()-> Vec<String> {
        let mut str_list = Vec::<String>::new();
        let ip_list = IP_LIST.read();
        for i in ip_list.iter(){
            str_list.push(format!("{}:{}",i.ip,i.port));
        }
        str_list
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
        client
            .put(url)
            .json(&self)
            .send()
            .unwrap();
    }
}
