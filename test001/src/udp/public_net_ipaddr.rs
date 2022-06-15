use spin::RwLock;
use std::sync::Arc;
use crate::apple::conf::ipadd;
use serde::{Deserialize, Serialize};
use super::{Msg,ChannelR,ChannelS,Types};
use crate::apple::conf;

lazy_static! {
    static ref IP:Arc<RwLock<String>> = {
        Arc::new(RwLock::new(ipadd::Conf::local_server()))
    };
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PublicNetIP{
    ip:String,
    port:u16
}
impl PublicNetIP {
    pub fn down()->String{
        IP.read().to_string()
    }

    pub fn upload(&self){
        let mut ipal = IP.write();
        *ipal = format!("{}:{}",self.ip,self.port);
        println!("New IP is:{}",ipal);
    }


    pub fn access(){
        let a = ipadd::Conf::remote_server();
        Msg
    }

}


