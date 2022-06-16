#[macro_use]
extern crate lazy_static;
use gdnative::prelude::*;
use std::thread;
mod apple;
// mod client;
mod udp;

use udp::PublicNetIP;
// use apple::conf::buffer;
// use apple::model::{cmsg,stats};


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
        if let Err(err) = PublicNetIP::access(){
            godot_print!("获取公网ip出错：{:?}",err);
        };
        godot_print!("Rust-启动udpserver");
        thread::spawn(move || {
            udp::start();
        });
    }

    // #[export]
    // fn udp_receive_stats(&self, _owner: &Node) {
    //     let rx = buffer::RqStats::get_receiver();
    //     if rx.is_empty() {
    //         godot_print!("角色属性数据没有接收值");
    //     } else {
    //         let a = rx.recv().unwrap();
    //         godot_print!("接收到角色属性数据：{:?}", a);
    //     }
    // }

    // #[export]
    // fn udp_receive_action(&self, _owner: &Node) {
    //     let rx = buffer::RqAction::get_receiver();
    //     if rx.is_empty() {
    //         godot_print!("角色行为数据没有接收值");
    //     } else {
    //         let a = rx.recv().unwrap();
    //         godot_print!("接收到角色行为数据：{:?}", a);
    //     }
    // }

    // #[export]
    // fn udp_send_stats(
    //     &self,
    //     _owner: &Node,
    //     class: Option<String>,
    //     name: Option<String>,
    //     max_hp: i32,
    //     max_mp: i32,
    //     phy: i32,
    //     spi: i32,
    //     agile: i32,
    //     speed: i32,
    //     sight: i32,
    //     lucky: i32,
    //     weight: i32,
    //     lv: i32,
    //     exp: i32,
    // ) {
    //     let h = cmsg::Head {
    //         cs: cmsg::Class::STATS,
    //         seq: 0,
    //         next: -1,
    //     };

    //     let s = stats::Stats {
    //         class,
    //         name,
    //         max_hp,
    //         max_mp,
    //         phy,
    //         spi,
    //         agile,
    //         speed,
    //         sight,
    //         lucky,
    //         weight,
    //         lv,
    //         exp,
    //     };
    //     let tx = buffer::SqMsg::get_sender();
    //     let data = cmsg::Data::new(h, Some(s));
    //     let buf = data.make().unwrap();

    //     let a = buffer::OthersIPA::get();
    //     for i in a.iter() {
    //         let cmsg = cmsg::CMsg {
    //             ipadd: i.to_string(),
    //             buf,
    //         };
    //         if let Err(err) = tx.send(cmsg) {
    //             godot_print!("{:?}", err);
    //         };
    //     }
    // }

    #[export]
    fn ipa(&self, _owner: &Node) -> String {
        PublicNetIP::down()
    }

    // #[export]
    // fn login(&self, _owner: &Node, account: String, password: String) {
    //     let usr = client::user::LoginUser{account, password};
    //     usr.login();
    // }
}

fn init(handle: InitHandle) {
    handle.add_class::<Signal>();
}

godot_init!(init);
