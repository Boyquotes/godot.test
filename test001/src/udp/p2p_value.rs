use super::{Launch, Msg};
use flume::{unbounded, Receiver, Sender};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::apple::Result;
use super::{Cursor,Sign};

lazy_static! {
    static ref P2PQUEUE: (Sender<P2PValue>, Receiver<P2PValue>) = unbounded();
}

/**
 * 用户行为接收通道
 */
pub struct P2PQueue;
impl P2PQueue {
    pub fn set() -> Sender<P2PValue> {
        P2PQUEUE.0.clone()
    }
    pub fn get() -> Receiver<P2PValue> {
        P2PQUEUE.1.clone()
    }

    pub async fn recv_to_queue(msg:Msg)->Result<()>{
        let value: P2PValue = msg.get_object()?;
        Self::set().send_async(value).await?;
        Ok(())
    }

    pub fn get_to_value()-> Result<P2PValue>{

        if Self::get().is_empty() {
            Ok(P2PValue::None)
        }else{  
            let a = Self::get().recv()?;
            Ok(a)
        }
    }




}

#[derive(Debug, Serialize, Deserialize)]
pub enum P2PValue {
    Data(Value),
    None
    
}
impl P2PValue {
    pub fn new(data:Value) -> Self {
        Self::Data(data)
    }

    pub fn to_string(&self)-> Result<Option<String>>{
        match self {
            P2PValue::Data(v) => {
                let a = serde_json::to_string(v)?;
                return Ok(Some(a))
            },
            P2PValue::None => {
                return Ok(None)
            },
        }
        
        
    }

    pub fn send_action_new(&self)->Result<()>{
        
        for (ipa,sign) in Cursor::find().data{
            let type1: String = "ACTION-NEW".to_owned();
            if let Sign::Rigth(port)=sign{
                let mut msg = Msg::new(ipa.ip, port, type1.clone());
                msg.set_object(self)?;
                let buf = msg.to_buf()?;
                // ChannelS::set().send(buf)?;
                Launch::ready(buf)?
            }
        }

        Ok(())
    }    
    
    
    pub fn _test() -> Self {
        let data = json!({
            "Class": "Player",
            "Name": "玩家名称",
            "Status":0,
            "Position": (0.0,0.0),
            "Speed": (0.0,0.0),
            "Back":(0.0,0.0),
            "HP":10,
            "MP": 0,
            "ATN": 1,
            "INT":0,
        });
        Self::Data(data)
    }
}
