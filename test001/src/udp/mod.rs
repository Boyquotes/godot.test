use crate::apple::queue;
use crate::godot_print;
pub use queue::{Buf, ChannelR, ChannelS, Msg};
use tokio::time::{sleep, Duration};
mod p2p_value;
mod room;
mod public_net_ipaddr;
mod receive_and_send;
mod receive_process;
mod p2p_ip_map;

pub use p2p_value::{P2PQueue,P2PValue};
pub use room::{NetIP, RoomIP};
pub use public_net_ipaddr::PublicNetIP;
pub use receive_and_send::Task;
pub use p2p_ip_map::{IpMap,Cursor,Data,Sign};

#[tokio::main]
pub async fn start() {
    let task1 = Task::new().await.unwrap();
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
        while let Ok(msg) = receive_process::Task::begin().await {
            // godot_print!("Rust->处理当前数据:{:?}", msg);
            
        }
    });

    // p2p探测
    tokio::spawn(async move {
        godot_print!("Rust->启动p2p探测。。。");
        while let Ok(()) = IpMap::start().await {
            godot_print!("Rust->p2p探测执行");      
            
        }
    });



    loop {
        godot_print!("Rust->当前玩家：{:?}", RoomIP::get_player());
        godot_print!("Rust->玩家映射：{:?}", Cursor::find());
        godot_print!("Rust->当前公网：{:?}", PublicNetIP::read());
       
        sleep(Duration::from_secs(5)).await;
    }
}
