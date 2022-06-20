use super::{ChannelR,Msg};
use super::public_net_ipaddr::PublicNetIP;
use crate::apple::Result;


pub struct Task;
impl Task {
    pub async fn begin()->Result<Msg>{
        let rx = ChannelR::get();
        let buf = rx.recv_async().await?;
        let msg = buf.to_msg()?;

        if let Some(tp) = msg.get_type(){
            match &tp as &str {
                "IP" => {
                    let ip = msg.get_object::<PublicNetIP>().unwrap();
                    ip.write();
                },
                "ROOM" => {
                    
                }
                "ACTION" => {
                    
                }
                _ => (),
            }
        }
        Ok(msg)
    }
}


