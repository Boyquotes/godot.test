#[macro_use]
extern crate lazy_static;
use gdnative::prelude::*;
mod apple;
mod udp;
use std::{thread, time};
use udp::{ RoomIP, P2PQueue,P2PValue};

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


    // 加入房间
    #[export]
    fn player_join_room(&self, _owner: &Node,key:String) {
        if let Ok(msg) = RoomIP::ask(key.clone()){
            RoomIP::key_set(key);
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



#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {




        thread::spawn(move || {
            println!("Rust->启动udpserver");
            udp::start();
        });

    }
}

