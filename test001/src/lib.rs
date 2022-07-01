#[macro_use]
extern crate lazy_static;
use gdnative::prelude::*;
mod apple;
mod udp;
use std::{thread, time};
use udp::{ PublicNetIP, RoomIP, P2PQueue,P2PValue};

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
            match PublicNetIP::public_net_ip() {
                Ok(msg)=>{
                    godot_print!("Rust->第{}次发送IP-ASK:{:?}...", &n, msg);
                }
                Err(e)=>{
                    godot_print!("Rust->第{}次发送，错误:{:?}...",&n, e);
                }  
            }
            n = n + 1;
            let ten_millis = time::Duration::from_millis(1000);
            thread::sleep(ten_millis);

            if let Some(ip) = PublicNetIP::read() {
                break ip.to_string();
            }else{
                continue;
            }
        }
    }


    // 加入房间
    #[export]
    fn player_join_room(&self, _owner: &Node,key:String) {
        if let Ok(msg) = RoomIP::ask(key){

            godot_print!("Rust->发送加入房间请求{:?}",msg);
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
        match P2PQueue::get_to_value(){
            Err(e)=>{
                godot_print!("Rust->获取value值错误：{:?}",e);
                return None
            }
            Ok(v)=>{
                match v.to_string() {
                    Err(e)=>{
                        godot_print!("Rust->获取value值错误：{:?}",e);
                        return None
                    }
                    Ok(rst)=>{
                        return rst
                    }
                }
            }
        }
    
    }

    #[export]
    fn p2p_send(&self, _owner: &Node,jstr:String ) {
        match serde_json::from_str(&jstr) {
            Ok(v)=>{
                let p2p_value = P2PValue::new(v);
                if let Err(e) = p2p_value.send_action_new(){
                    godot_print!("Rust->发送行为错误{:?}",e);
                }
            }
            Err(e)=>{
                godot_print!("Rust->解析错误{:?}",e);
            }
        }
        
    }

}

fn init(handle: InitHandle) {
    handle.add_class::<Signal>();
}
godot_init!(init);
