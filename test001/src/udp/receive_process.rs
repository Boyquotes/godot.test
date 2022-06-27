use super::p2p_value::{P2PQueue, P2PValue};
use super::player::{NetIP,RoomIP};
use super::public_net_ipaddr::PublicNetIP;
use super::{ChannelR, Msg};
use crate::apple::Result;

pub struct Task;
impl Task {
    pub async fn begin() -> Result<Msg> {
        let rx = ChannelR::get();
        let buf = rx.recv_async().await?;
        let msg = buf.to_msg()?;

        if let Some(tp) = msg.get_type() {
            match &tp as &str {
                "IP-RSP" => {
                    let ip: PublicNetIP = msg.get_object()?;
                    ip.write();
                }
                "ROOM-RSP" => {
                    let ip_list: Vec<NetIP> = msg.get_object()?;
                    let room = RoomIP{ip_list};
                    room.put_player();
                }
                "ACTION-NEW" => {
                    let value: P2PValue = msg.get_object()?;
                    P2PQueue::set().send_async(value).await?;
                }
                _ => (),
            }
        }
        Ok(msg)
    }
}
