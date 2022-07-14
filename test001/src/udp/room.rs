use crate::apple::conf::ipadd;
use serde::{Deserialize, Serialize};
use spin::RwLock;
use crate::apple::Result;
use std::{net::SocketAddr, str::FromStr, sync::Arc};
use super::{NetIP,Launch, Msg,Buf,Cursor,IpMap,Sign};



lazy_static! {
    static ref ROOM: Arc<RwLock<Room>> = {
        let room = Room::new();
        Arc::new(RwLock::new(room))
    };


    static ref KEY: Arc<RwLock<Option<String>>> = {
        Arc::new(RwLock::new(None))
    };

}



/**
 * 房间玩家IP列表
 */
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Room {
    pub myself: Option<NetIP>,
    pub ip_list: Vec<NetIP>
}

impl Room {
    pub fn new() -> Self {
        let ip_list = Vec::new();
        Self {myself:None,ip_list}
    }

    pub fn get_myself()->Option<NetIP>{
        let room = ROOM.read();
        room.myself.clone()
    }

    pub fn get_player() -> Room {
        let room = ROOM.read();
        room.clone()
    }

    pub fn save(&self) {
        let mut ipal = ROOM.write();
        *ipal = self.clone();
    }

    pub fn key_set(key:String){
        let mut rkey = KEY.write();
        *rkey = Some(key);
    }

    pub fn key_get()->Option<String>{
        let key = KEY.read();
        key.clone()
    }

    /** 
     * 加入房间请求
    */
    pub fn ask(key:String)-> Result<()>{
        let url = ipadd::URL::remote_server();
        let ipa = SocketAddr::from_str(&url)?;
        let mut msg = Msg::new(ipa.ip().to_string(), ipa.port(), "ROOM-ASK".to_owned());
        msg.insert("ROOM".to_owned(), key);
        let buf = msg.to_buf()?;
        Launch::ready(buf)?;
        Ok(())
    }

    /**
    * 收到数据并回复
    * msg 接收到的消息
    */ 
    pub fn rsp(msg:Msg)->Result<()>{
        let room: Room = msg.get_object()?;
        // 更新映射表
        if let Some(ipn) = room.myself.clone(){
            Cursor::set_host(ipn)
        }


        for i in room.ip_list.clone(){
            if !Cursor::exist(&i){
                let ipm = IpMap::new(i,Sign::Test);
                Cursor::replace_one(ipm);
                // godot_print!("Rust->更新房间信息{:?}",room);
            }
        }
        
        room.save();
        Ok(())
    }

    /**
    * 回复确认已收到
    */ 
    pub fn check(buf:Buf)->Result<()>{
        let mut msg = Msg::new(buf.ip.clone(), buf.port, "ROOM-CHK".to_owned());
        msg.insert("MD5".to_owned(), buf.get_md5());
        // godot_print!("Rust->收到房间信息,回复确认{:?}",msg);
        let buf = msg.to_buf()?;
        // ChannelS::set().send(buf)?;
        Launch::ready(buf)?;
        Ok(())
    }
}
