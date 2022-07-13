mod room;
mod domain;
mod receive_and_send;
mod receive_process;
mod p2p_value;
use serde::{Deserialize, Serialize};
pub use crate::apple::udp_channel::{Launch, Accept, Msg, Buf};
use domain::{Cursor,IpMap,Domain,Sign};
use receive_and_send::UdpServer;
use receive_process::Process;
pub use room::Room;
pub use p2p_value::{P2PValue,P2PQueue};


mod start;
pub use start::start;
/** 
 * 公网IP地址
 */
#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Clone)]
pub struct NetIP {
    pub ip: String,
    pub port: u16,
}
impl NetIP {
    pub fn new(ip: String, port: u16) -> Self { Self { ip, port } }
}