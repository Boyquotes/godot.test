use crate::apple::queue;
use crate::godot_print;
pub use queue::{Buf, ChannelR, ChannelS, Msg};
use tokio::time::{sleep, Duration};
mod p2p_value;
mod player;
mod public_net_ipaddr;
mod receive_and_send;
mod receive_process;
pub use p2p_value::{P2PQueue,P2PValue};
pub use player::{NetIP, RoomIP};
pub use public_net_ipaddr::PublicNetIP;
pub use receive_and_send::Task;

#[tokio::main]
pub async fn start() {
    let task1 = Task::new().await.unwrap();
    let task2 = task1.clone();

    // 发送数据
    tokio::spawn(async move {
        while let Ok(msg) = task1.udp_sender().await {
            godot_print!("Rust->发送当前数据{:?}", msg);
        }
    });
    // 接收数据
    tokio::spawn(async move {
        while let Ok(msg) = task2.udp_accept().await {
            godot_print!("Rust->接收当前数据:{:?}", msg);
        }
    });

    // 处理数据
    tokio::spawn(async move {
        while let Ok(msg) = receive_process::Task::begin().await {
            godot_print!("Rust->处理当前数据:{:?}", msg);
        }
    });

    loop {
        sleep(Duration::from_secs(2)).await;
        let len1 = ChannelS::get().len();
        godot_print!("Rust->当前待发送数据为：{:?}", len1);
        sleep(Duration::from_secs(2)).await;
        let len2 = ChannelR::get().len();
        godot_print!("Rust->当前待处理数据为：{:?}", len2);
    }
}
