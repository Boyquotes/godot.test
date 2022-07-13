use crate::apple::udp_channel;
use crate::godot_print;
pub use udp_channel::{Launch, Accept, Msg,Buf};
use tokio::time::{sleep, Duration};
use super::*;


#[tokio::main]
pub async fn start() {
    let task1 = UdpServer::new().await.unwrap();
    let task2 = task1.clone();

    // 发送数据
    tokio::spawn(async move {
        while let Ok(msg) = task1.udp_sender().await {
            // godot_print!("Rust->发送当前数据{:?}", msg);
        }
    });
    // 接收数据
    tokio::spawn(async move {
        while let Ok(msg) = task2.udp_accept().await {
            // godot_print!("Rust->接收当前数据:{:?}", msg);
        }
    });

    // 处理数据
    tokio::spawn(async move {
        while let Ok(msg) = Process::begin().await {
            // godot_print!("Rust->处理当前数据:{:?}", msg);
            
        }
    });

    // p2p探测
    tokio::spawn(async move {
        godot_print!("Rust->启动p2p探测。。。");
        while let Ok(()) = Domain::start().await {
            // godot_print!("Rust->p2p探测执行");
                
            
        }
    });


    // room 心跳
    tokio::spawn(async move {
        loop {
            if let Some(key) = Room::key_get(){
                if let Ok(()) = Room::ask(key){
                    godot_print!("Rust-> 心跳发送")
                }
            }
            sleep(Duration::from_secs(30)).await;
        }
    });



    loop {
        println!("Rust->当前玩家：{:?}", Room::get_player());
        println!("Rust->玩家映射：{:?}", Cursor::find());

       
        sleep(Duration::from_secs(2)).await;
    }
}
