#[macro_use]
extern crate lazy_static;
use gdnative::prelude::*;
mod apple;
mod udp;
use serde_json::Value;
use std::{thread, time};
use udp::{NetIP, PublicNetIP, RoomIP, P2PValue,ChannelS};

#[derive(NativeClass)]
#[inherit(Node)]
struct Signal;

#[methods]
impl Signal {
    fn new(_owner: &Node) -> Self {
        Signal
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        thread::spawn(move || {
            godot_print!("Rust->启动udpserver");
            udp::start();
        });
    }

    #[export]
    fn public_net_ip_ask(&self, _owner: &Node) -> String {
        let mut n = 1;
        loop {
            if let Ok(msg) = PublicNetIP::public_net_ip() {
                godot_print!("Rust->第{}次发送IP-ASK:{:?}...", &n, msg);
                n = n + 1;
            }
            let ten_millis = time::Duration::from_millis(1000);
            thread::sleep(ten_millis);

            if let Some(ip) = PublicNetIP::read() {
                return ip.to_string()
            }
        }
    }


    // 加入房间
    #[export]
    fn player_join_room(&self, _owner: &Node,key:String) {
        if let Ok(msg) = RoomIP::join(key){
            godot_print!("{:?}",msg);
        };
 
    }

    #[export]
    fn get_stats(&self, _owner: &Node) {
        godot_print!("Rust->这里是角色属性");
    }

    #[export]
    fn set_stats(&self, _owner: &Node) {
        godot_print!("Rust->这里是角色属性");
    }


    #[export]
    fn p2p_recv(&self, _owner: &Node) -> Option<String> {
        let value = match P2PValue::recv() {
            Some(rst)=> rst,
            None=> {
                return None;
            }
        };
        value.to_string()

    }

    #[export]
    fn p2p_send(&self, _owner: &Node,jstr:String ) {
        let a = serde_json::from_str::<Value>(&jstr).unwrap();

        let pvalue = P2PValue::new(a);
     
       
        pvalue.send()
    }

    #[export]
    fn test_read_ip(&self, _owner: &Node) -> Option<String> {
        if let Some(rst) = PublicNetIP::read() {
            Some(rst.to_string())
        } else {
            None
        }
    }

    #[export]
    fn test_read_ip_list(&self, _owner: &Node) -> Vec<String> {
        RoomIP::get_player_to_string()
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Signal>();
}
godot_init!(init);
