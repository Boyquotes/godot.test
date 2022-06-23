use serde::{Deserialize, Serialize};
use super::queue::{Msg,ChannelS};
use super::player_net_ipaddr::PlayerNetIP;
use flume::{unbounded, Receiver, Sender};
use crate::godot_print;
// use serde_json::Value;

lazy_static! {
    static ref ACTRCDE:(Sender<PlayerAction>,Receiver<PlayerAction>) = unbounded();
}


/**
 * 用户行为接收通道
 */
pub struct ActionQ;
impl ActionQ {
    pub fn set()->Sender<PlayerAction>{
        ACTRCDE.0.clone()
    }
    pub fn get()->Receiver<PlayerAction>{
        ACTRCDE.1.clone()
    }
}


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PlayerAction {
    status: u8,
    position: [f64; 2],
    speed: [f64; 2],
    back: [f64; 2],
    hp: i32,
    mp: i32,
    atn: i32,
    int: i32,
}

impl PlayerAction {
    pub fn new(status: u8, position:[f64; 2] , speed: [f64; 2], back: [f64; 2], hp: i32, mp: i32, atn: i32, int: i32) -> Self
    { 
        Self { status, position, speed, back, hp, mp, atn, int } 
    }

    pub fn recv()-> Option<String>{
        let recv = ActionQ::get();
        if !recv.is_empty(){
            let act = ActionQ::get().recv().unwrap();
            
            let serialized = serde_json::to_string(&act).unwrap();

            godot_print!("接收到角色行为数据{:?}",serialized);
            return Some(serialized)
        };
        None
    }

    pub fn test() -> Self
    { 
        let status = 0;
        let position = [0.0,1.0];
        let speed=[0.0,1.0];
        let back=[0.0,1.0];
        let hp=10;
        let mp=1;
        let atn=1;
        let int=0;
        Self { status, position, speed, back, hp, mp, atn, int } 
    }

    pub fn send(&self){
        let a = PlayerNetIP::get_list();
        for i in a{
            let type1 = "ACTION-NEW".to_owned();
            let mut msg = Msg::new(i.ip,i.port,type1);
            msg.set_object(self).unwrap();
            let buf = msg.to_buf().unwrap();
            ChannelS::set().send(buf).unwrap();  
        }
    }

}

// "action" : {
//     "Class" : "Player",
//     "Name" : "玩家1",
//     "Status" : 0,
//     "Position" : {
//         "x" : 0,
//         "y" : 0
//     },
//     "Speed" : {
//         "x" : 0,
//         "y" : 0
//     },
//     "Back" : {
//         "x" : 0,
//         "y" : 0
//     },
//     "HP" : 10,
//     "MP" : 0,
//     "ATN" : 1,
//     "INT" : 0
// },


