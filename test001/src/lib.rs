#[macro_use]
extern crate lazy_static;
use gdnative::prelude::*;
use std::thread;
mod apple;
mod udp;
use udp::PublicNetIP;

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

    // #[export]
    // fn login(&self, _owner: &Node, account: String, password: String) {
    //     let usr = client::user::LoginUser{account, password};
    //     usr.login();
    // }

    #[export]
    fn get_stats(&self, _owner: &Node) {
        godot_print!("这里是角色属性");

    }

    #[export]
    fn set_stats(&self, _owner: &Node) {
        godot_print!("这里是角色属性");

    }

    #[export]
    fn udp_receive_action(&self, _owner: &Node) {
        godot_print!("角色行为数据");
    }

    #[export]
    fn access(&self, _owner: &Node) {
        if let Ok(buf) = PublicNetIP::access(){
            godot_print!("发送公网请求{:?}",buf.to_msg());
        };
    }
        
    #[export]
    fn ipa(&self, _owner: &Node) -> String {
        let ip =PublicNetIP::down();
        godot_print!("获取公网ip:{:?}",ip);
        ip
    }

}

fn init(handle: InitHandle) {
    handle.add_class::<Signal>();
}

godot_init!(init);
