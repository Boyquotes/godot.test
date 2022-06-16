use spin::RwLock;
use crate::apple::conf::ipadd;
use serde::{Deserialize, Serialize};
use super::{Msg,ChannelR,ChannelS,Types};
use std::{net::SocketAddr, sync::Arc, str::FromStr};
use crate::apple::Result;


lazy_static! {
    static ref IP:Arc<RwLock<String>> = {
        Arc::new(RwLock::new(ipadd::URL::local_server()))
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

    pub fn access() -> Result<()> {
        let url = ipadd::URL::remote_server();
        let add = SocketAddr::from_str(&url)?;
        let msg = Msg::new(add.ip().to_string(),add.port(),"IP".to_owned());
        let buf = msg.to_buf()?;
        ChannelS::set().send(buf)?;
        Ok(())
    }

}


