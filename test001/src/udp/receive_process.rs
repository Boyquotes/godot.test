use super::p2p_value::{P2PQueue};
use super::room::RoomIP;
use super::public_net_ipaddr::PublicNetIP;
use super::p2p_ip_map::IpMap;
use super::{ChannelR, Msg};
use crate::apple::Result;
use crate::godot_print;

pub struct Task;
impl Task {
    pub async fn begin() -> Result<Msg> {
        let rx = ChannelR::get();
        let buf = rx.recv_async().await?;
        let msg = buf.to_msg()?;

        if let Some(tp) = msg.get_type() {
            match &tp as &str {
                "IP-RSP" => PublicNetIP::write(msg.clone())?,
                "ROOM-RSP" => {
                    RoomIP::rsp(msg.clone())?
                }
                "P2P-ASK" => {
                    godot_print!("收到P2P请求========================================================>{:?}",msg);
                    // let _ = IpMap::ask(msg.clone());
                    // IpMapList::rsp(msg.clone())?
                }

                // "P2P-RSP" => {
                //     godot_print!("收到P2P回复========================================================>{:?}",msg);
                //     IpMapList::rsp(msg.clone())?
                // }
                    
                // "P2P-CHK" => IpMapList::check2(msg.clone())?,
                // "P2P-CHK" => {
                //     godot_print!("收到P2P确认{:?}",msg);
                // },
                "ACTION-NEW" => P2PQueue::recv_to_queue(msg.clone()).await?,
                _ => (),
            }
        }
        Ok(msg)
    }
}
