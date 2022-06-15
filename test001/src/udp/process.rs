use super::receive_and_send::{ChannelR,Msg,Types};
use super::public_net_ipaddr::PublicNetIP;

pub struct Task;

impl Task {
    pub fn start(){

        // while let Err(err) = ChannelR::get().recv() {
        //     println!("{:?}", err);
        // }

        loop{
            let buf = ChannelR::get().recv().unwrap();
            let msg = buf.to_msg().unwrap();
            match msg.get_type().unwrap() {
                Types::IP => {
                    let ip = msg.get_object::<PublicNetIP>().unwrap();
                    ip.upload()
                }
                Types::ROOM => todo!(),
                Types::STATS => todo!(),
                Types::ACTION => todo!(),
                Types::NULL => todo!(),
            }
        }
    }
}

