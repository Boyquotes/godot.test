#[macro_use]
extern crate lazy_static;
use gdnative::prelude::*;
mod apple;
mod udp;
use serde_json::Value;
use std::{thread, time};
use udp::{PlayerNetIP, PublicNetIP, RoomIP, ACT};

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
            godot_print!("Rust-启动udpserver");
            udp::start();
        });
    }

    #[export]
    fn public_net_ip_ask(&self, _owner: &Node) -> String {
        let mut n = 1;
        loop {
            if let Ok(msg) = PublicNetIP::public_net_ip() {
                godot_print!("第{}次发送IP-ASK:{:?}...", &n, msg);
                n = n + 1;
            }
            let ten_millis = time::Duration::from_millis(1000);
            thread::sleep(ten_millis);
            if let Some(ip) = PublicNetIP::read() {
                break ip.to_string();
            }
        }
    }

    #[export]
    fn player_join_room(&self, _owner: &Node) -> bool {
        if let Some(ipa) = PublicNetIP::read() {
            let key = 0;
            let ip = ipa.ip;
            let port = ipa.port;
            let room = RoomIP::new(key, ip, port);
            room.join();
            godot_print!("玩家进入房间：{:?}", room);
            true
        } else {
            godot_print!("未获得公网IP，进入房间失败...");
            false
        }
    }

    #[export]
    fn get_stats(&self, _owner: &Node) {
        godot_print!("这里是角色属性");
    }

    #[export]
    fn set_stats(&self, _owner: &Node) {
        godot_print!("这里是角色属性");
    }

    #[export]
    fn recv_action(&self, _owner: &Node) -> Option<String> {
        ACT::recv()
    }

    #[export]
    fn send_player_action(&self, _owner: &Node, jstr: String) {
        if let Ok(rst) = serde_json::from_str::<Value>(&jstr) {
            let obj = ACT::new(rst);
            obj.send();
        };
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
        PlayerNetIP::get_list_to_string()
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Signal>();
}
godot_init!(init);
