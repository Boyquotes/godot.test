use gdnative::prelude::godot_print;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::UdpSocket;
use crate::apple::model;
use crate::apple::conf;
// use crate::apple::Result;

pub mod server;