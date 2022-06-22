use crate::apple::Result;
use super::{ChannelR,Msg};
use super::public_net_ipaddr::PublicNetIP;
use super::player_net_ipaddr::PlayerNetIP;
use super::player_action_new::{PlayerAction,ActionQ};


pub struct Task;
impl Task {
    pub async fn begin()->Result<Msg>{
        let rx = ChannelR::get();
        let buf = rx.recv_async().await?;
        let msg = buf.to_msg()?;

        if let Some(tp) = msg.get_type(){
            match &tp as &str {
                "IP-RSP" => {
                    let ip:PublicNetIP = msg.get_object().unwrap();
                    ip.write();
                },
                "ROOM-NEW" => {
                    let ip_list:Vec<PlayerNetIP> = msg.get_object().unwrap();
                    PlayerNetIP::set_list(ip_list)
                }
                "ACTION-NEW" => {
                    let a:PlayerAction = msg.get_object().unwrap();
                    ActionQ::set().send_async(a).await.unwrap();
                }
                _ => (),
            }
        }
        Ok(msg)
    }
}


