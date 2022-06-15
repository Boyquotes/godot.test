// use gdnative::prelude::godot_print;
// use std::{net::SocketAddr, sync::Arc};
// use tokio::net::UdpSocket;
// use crate::apple::model;

use std::thread;

mod process;
mod receive_and_send;
mod public_net_ipaddr;
pub use receive_and_send::{Buf,Msg,ChannelR,ChannelS,Types};
pub use public_net_ipaddr::PublicNetIP;


pub fn start(){
    thread::spawn(move || {
        receive_and_send::Task::start();
    });

    thread::spawn(move || {
        process::Task::start();
    });
}