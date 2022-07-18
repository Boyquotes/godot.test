use super::*;
use crate::apple::Result;
use crate::godot_print;

pub struct Process;
impl Process {
    pub async fn begin() -> Result<Msg> {
        let buf = Accept::get_async().await?;
        let msg = buf.to_msg()?;
        godot_print!("Rust->收到UDP消息=>{:?}",msg);
        if let Some(tp) = msg.get_type() {
            match &tp as &str {
                "ROOM-RSP" => {
                    Room::rsp(msg.clone())?;
                    Room::check(buf.clone())?;
                }
                "P2P-ASK" => {
                    Domain::accept(msg.clone()).await?;
                    
                }
                "P2P-RSP" => {
                    Domain::rsp(msg.clone())?
                }
                "ACTION-NEW" => {
                    P2PQueue::recv_to_queue(msg.clone()).await?
                }
                _ => (),
            }
        }
        Ok(msg)
    }
}
