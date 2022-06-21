use spin::RwLock;
use crate::apple::conf::ipadd;
use serde::{Deserialize, Serialize};
use super::{Msg,ChannelS};
use std::{net::SocketAddr, sync::Arc, str::FromStr};
use crate::apple::Result;



lazy_static! {
    static ref IP:Arc<RwLock<Option<PublicNetIP>>> = {
        // let ipadd = ipadd::URL::local_host();
        // let ipadd = SocketAddr::from_str(&ipadd).unwrap();
        // let ip = ipadd.ip().to_string();
        // let port = ipadd.port();
        // let public_net_ip = PublicNetIP::new(ip,port);
        // Arc::new(RwLock::new(public_net_ip))
        Arc::new(RwLock::new(None))
    };
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PublicNetIP{
    pub ip:String,
    pub port:u16
}

impl PublicNetIP {
    pub fn new(ip: String, port: u16) -> Self { Self { ip, port } }
    // 访问外网服务，得到公网地址
    
    pub fn public_net_ip() -> Result<Msg> {
        let url = ipadd::URL::remote_server();
        let ipa = SocketAddr::from_str(&url)?;
        let msg = Msg::new(ipa.ip().to_string(),ipa.port(),"IP-ASK".to_owned());
        let buf = msg.to_buf()?;
        ChannelS::set().send(buf)?;
        Ok(msg)
    }

    pub fn read()-> Option<PublicNetIP> {
        let _rst = Self::public_net_ip();
        let ipa = IP.read();
        if let Some(rst) = ipa.as_ref() {
            let ip = rst.ip.clone();
            let port = rst.port;
            Some(Self{ip,port})
        } else {
            None
        }
    }

    pub fn to_string(&self)-> String {
        format!("{}:{}",self.ip,self.port)
    }

    pub fn write(self){
        let mut ipal = IP.write();
        *ipal = Some(self);
    }



}


