use super::p2p_value::{P2PQueue};
use super::player::RoomIP;
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
                "IP-RSP" => PublicNetIP::write(msg.clone())?,
                "ROOM-RSP" => RoomIP::rsp(msg.clone())?,
                "ACTION-NEW" => P2PQueue::recv_to_queue(msg.clone()).await?,
                _ => (),
            }
        }
        Ok(msg)
    }
}
