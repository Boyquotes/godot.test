use crate::apple::queue;
use crate::godot_print;
pub use queue::{Buf, ChannelR, ChannelS, Msg};
use tokio::time::{sleep, Duration};
mod p2p_value;
mod room;
mod public_net_ipaddr;
mod receive_and_send;
mod receive_process;
mod player_net_ipaddr;

pub use p2p_value::{P2PQueue,P2PValue};
pub use room::{NetIP, RoomIP};
pub use public_net_ipaddr::PublicNetIP;
pub use receive_and_send::Task;
pub use player_net_ipaddr::PlayerNetMapList;

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

    loop {
        sleep(Duration::from_secs(2)).await;

        
        godot_print!("Rust->当前玩家：{:?}", RoomIP::get_player());
        godot_print!("Rust->玩家映射：{:?}", PlayerNetMapList::get());
    }
}
