use super::player_net_ipaddr::PlayerNetIP;
use super::{ChannelS, Msg};
use flume::{unbounded, Receiver, Sender};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

lazy_static! {
    static ref ACTRCDE: (Sender<ACT>, Receiver<ACT>) = unbounded();
}

/**
 * 用户行为接收通道
 */
pub struct ActionQ;
impl ActionQ {
    pub fn set() -> Sender<ACT> {
        ACTRCDE.0.clone()
    }
    pub fn get() -> Receiver<ACT> {
        ACTRCDE.1.clone()
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ACT {
    data: Value,
}
impl ACT {
    pub fn new(data: Value) -> Self {
        Self { data }
    }

    pub fn recv() -> Option<String> {
        let recv = ActionQ::get();
        if !recv.is_empty() {
            let data = ActionQ::get().recv().unwrap();
            let jstr = serde_json::to_string(&data).unwrap();
            return Some(jstr);
        };
        None
    }

    pub fn send(&self) {
        for i in PlayerNetIP::get_list() {
            let type1 = "ACTION-NEW".to_owned();
            let mut msg = Msg::new(i.ip, i.port, type1);
            msg.set_object(self).unwrap();
            let buf = msg.to_buf().unwrap();
            ChannelS::set().send(buf).unwrap();
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
        Self { data }
    }
}
