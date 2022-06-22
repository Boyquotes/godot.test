#[macro_use]
extern crate lazy_static;
use gdnative::prelude::*;
use std::thread;
mod apple;
mod udp;
use udp::{PublicNetIP,RoomIP,PlayerNetIP,PlayerAction};

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

        if let Ok(msg) = PublicNetIP::public_net_ip(){
            godot_print!("发送IP-ASK:{:?}",msg);
        };
    }

    #[export]
    fn join_room(&self, _owner: &Node) -> bool {
        if let Some(ipa) = PublicNetIP::read(){
            let key = 0;
            let ip = ipa.ip;
            let port = ipa.port;
            let room =RoomIP::new(key,ip,port);
            godot_print!("加入房间：{:?}",room);
            room.join();
            true
        }else{
            godot_print!("加入房间失败...");
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
    fn recv_action(&self, _owner: &Node) {
        PlayerAction::recv();
    }

    #[export]
    fn send_action(&self, _owner: &Node) {
        godot_print!("发送角色行为数据");
        let testac = PlayerAction::test();
        testac.send()
    }
        
    #[export]
    fn read_ip(&self, _owner: &Node) -> Option<String> {     
        if let Some(rst) = PublicNetIP::read() {
            Some(rst.to_string())
        } else {
            None
        }
    }

    #[export]
    fn read_ip_list(&self, _owner: &Node) -> Vec<String> {     
        let rst = PlayerNetIP::get_list_to_string();
        rst
    }


}

fn init(handle: InitHandle) {
    handle.add_class::<Signal>();
}

godot_init!(init);
