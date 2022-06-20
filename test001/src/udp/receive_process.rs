use super::{ChannelR,Msg,Types};
use super::public_net_ipaddr::PublicNetIP;
use crate::apple::Result;


pub struct Task;
impl Task {
    pub async fn begin()->Result<Msg>{
        let buf = ChannelR::get().recv_async().await?;
        let msg = buf.to_msg()?;
        // match msg.get_type()? {
        //     Types::IP => {
        //         let ip = msg.get_object::<PublicNetIP>().unwrap();
        //         ip.upload();
        //     }
        //     Types::ROOM => todo!(),
        //     Types::STATS => todo!(),
        //     Types::ACTION => todo!(),
        //     Types::NULL => todo!(),
        // }
        Ok(msg)
    }
}


