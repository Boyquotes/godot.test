mod queue;
pub use queue::{ChannelR,ChannelS,Buf,Msg};
mod receive_process;
mod receive_and_send;
mod public_net_ipaddr;
mod player_net_ipaddr;
pub use receive_and_send::Task;
pub use public_net_ipaddr::PublicNetIP;
pub use player_net_ipaddr::RoomIP;
use tokio::time::{sleep, Duration};
use crate::godot_print;


#[tokio::main]
pub async fn start() {
    let task1 = Task::new().await.unwrap();
    let task2 = task1.clone();

    // 发送数据
    tokio::spawn(async move {
        while let Ok(msg) = task1.udp_sender().await {
            godot_print!("发送当前数据{:?}", msg);
        }
    });
    // 接收数据
    tokio::spawn(async move {
        while let Ok(msg) = task2.udp_accept().await {
            godot_print!("接收当前数据:{:?}",msg);
        }
    });

    // 处理数据
    tokio::spawn(async move {
        while let Ok(msg) = receive_process::Task::begin().await {
            godot_print!("处理当前数据:{:?}",msg);
        }
    });

    loop{
        sleep(Duration::from_secs(1)).await;
        let len1 = ChannelS::get().len();
        godot_print!("当前待发送数据为：{:?}",len1);
        sleep(Duration::from_secs(1)).await;
        let len2 = ChannelR::get().len();
        godot_print!("当前待处理数据为：{:?}",len2);
    }
}