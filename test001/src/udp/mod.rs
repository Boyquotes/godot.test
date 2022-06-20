mod channel;
pub use channel::{ChannelR,ChannelS,Buf,Msg,Types};
mod receive_process;
mod receive_and_send;
mod public_net_ipaddr;
pub use receive_and_send::Task;
pub use public_net_ipaddr::PublicNetIP;
use crate::godot_print;


#[tokio::main]
pub async fn start(){
    
    let task1 = Task::new().await.unwrap();
    let task2 = task1.clone();
    // 发送数据
    tokio::spawn(async move {
        while let Ok(msg) = task1.udp_sender().await {
            godot_print!("发送数据{:?}", msg);
        }
    });
    // 接收数据
    tokio::spawn(async move {
        while let Ok(msg) = task2.udp_accept().await {
            godot_print!("接收数据:{:?}",msg);
        }
    });

    // 处理数据
    while let Ok(msg) = receive_process::Task::begin().await {
        godot_print!("处理数据:{:?}",msg);
    }
}