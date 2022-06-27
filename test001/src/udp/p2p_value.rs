use super::player::RoomIP;
use super::{ChannelS, Msg};
use flume::{unbounded, Receiver, Sender};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct P2PValue {
    data: Value
}
impl P2PValue {
    pub fn new(data:Value) -> Self {Self {data}}

    pub fn recv() -> Option<Self> {
        if P2PQueue::get().is_empty() {
            None
        }else{  
            match P2PQueue::get().recv() {
                Ok(rst)=>Some(rst),
                Err(err)=>{
                    println!("{:?}",err);
                    None
                }
            }
        }
    }

    pub fn to_string(&self)-> Option<String>{
        match serde_json::to_string(&self.data){
            Ok(rst)=>Some(rst),
            Err(err)=>{
                println!("{:?}",err);
                None
            }
        }
    }

    pub fn send(&self) {
        let type1: String = "ACTION-NEW".to_owned();
        for i in RoomIP::get_player() {
            let mut msg = Msg::new(i.ip, i.port, type1.clone());
            if let Err(err) = msg.set_object(self){
                println!("{:?}",err);
            };
            if let Ok(buf)= msg.to_buf() {
                if let Err(err) = ChannelS::set().send(buf){
                    println!("{:?}",err);
                }
            }  
        }
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
        Self {data}
    }
}
