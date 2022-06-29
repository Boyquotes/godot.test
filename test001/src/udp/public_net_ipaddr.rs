use super::{ChannelS, Msg};
use crate::apple::conf::ipadd;
use crate::apple::Result;
use serde::{Deserialize, Serialize};
use spin::RwLock;
use std::{net::SocketAddr, str::FromStr, sync::Arc};

lazy_static! {
    static ref IP: Arc<RwLock<Option<PublicNetIP>>> = Arc::new(RwLock::new(None));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicNetIP {
    pub ip: String,
    pub port: u16,
}

impl PublicNetIP {
    // 访问外网服务，得到公网地址
    pub fn public_net_ip() -> Result<Msg> {
        let url = ipadd::URL::remote_server();
        let ipa = SocketAddr::from_str(&url)?;
        let msg = Msg::new(ipa.ip().to_string(), ipa.port(), "IP-ASK".to_owned());
        let buf = msg.to_buf()?;
        ChannelS::set().send(buf)?;
        Ok(msg)
    }

    pub fn read() -> Option<PublicNetIP> {
        let ipa = IP.read();
        if let Some(rst) = ipa.as_ref() {
            let ip = rst.ip.clone();
            let port = rst.port;
            Some(Self { ip, port })
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    pub fn write(msg:Msg)->Result<()> {
        let ip: PublicNetIP = msg.get_object()?;
        let mut ipal = IP.write();
        *ipal = Some(ip);
        Ok(())
    }
}
