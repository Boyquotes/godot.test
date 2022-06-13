use super::*;
use crate::apple::Result;
use model::{cmsg,room};
use conf::{buffer,ipadd};

/**
 * 获取公网IP
 */
pub fn get_ipa() -> String {
    let h = cmsg::Head {
        cs: cmsg::Class::IP,
        seq: 0,
        next: 1,
    };
    let data = cmsg::Data::new(h, Option::<String>::None);
    if let Ok(buf) = data.make() {
        // let tx = S_CMSG.0.clone();
        let tx = buffer::SqMsg::get_sender();
        // let ipadd = String::from("118.193.46.124:5002");
        let ipadd = ipadd::Conf::remote_server();
        let cmsg = cmsg::CMsg { ipadd, buf };
        if let Err(err) = tx.send(cmsg) {
            godot_print!("{:?}", err);
        };
    };
    // MyIPA.read().to_string()
    buffer::MyIPA::get()

}
/**
 * upd发送和接收服务
 */
#[tokio::main]
pub async fn udp_server_start() {
    let udps1 = match UdpServer::new().await {
        Ok(rst) => rst,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };
    let udps2 = Arc::new(udps1);
    let udps3 = udps2.clone();

    // 发送
    tokio::spawn(async move {
        while let Err(err) = udps3.udp_sender().await {
            println!("{:?}", err);
        }
    });

    // 接收

    loop {
        if let Err(err) = udps2.udp_accept().await {
            println!("{:?}", err);
        }
    }
}


#[derive(Debug,Clone)]
struct UdpServer {
    sock: UdpSocket,
}
impl UdpServer {
    /**
     * 创建 udp socket
     */
    async fn new() -> Result<Self> {
        let sock = UdpSocket::bind("0.0.0.0:8080".parse::<SocketAddr>()?).await?;
        Ok(Self { sock })
    }

    /**
     * 从通道接收数据，然后由udp socket发送数据
     */
    async fn udp_sender(&self) -> Result<()> {
        let rx = buffer::SqMsg::get_receiver();
        let cmsg::CMsg { ref buf, ref ipadd } = rx.recv_async().await?;
        let len = self.sock.send_to(&buf.bytes, ipadd).await?;
        godot_print!("发送成功，发送数据至{:?} ", ipadd);
        Ok(())
    }

    /**
     *  udp socket接收数据
     */
    async fn udp_accept(&self) -> Result<()> {
        let mut buf = cmsg::Buf::new();
        let (len, addr) = self.sock.recv_from(&mut buf.bytes).await?;
        godot_print!("upd接收数据长度:{:?},发送源地址：{:?}", len, addr);
        let head = buf.get_head()?;
        match head.cs {
            cmsg::Class::IP => {
                // 获取公网IP
                let ipa: String = buf.get_content()?;
                println!("得到公网地址：{:?}", ipa);
                buffer::MyIPA::set(ipa);
            }
            cmsg::Class::ROOM => {
                // 房间数据
                let r: room::Room = buf.get_content()?;
                let ipa_list = r.ip();
                buffer::OthersIPA::set(ipa_list.clone());
                godot_print!("收到更新房间消息：{:?}", ipa_list);
            }
            cmsg::Class::STATS => {
                // 角色状态
                let tx = buffer::RqStats::get_sender();
                tx.send_async(buf.get_content()?).await?;
            }
            cmsg::Class::ACTION => {
                // 角色状态
                let tx = buffer::RqStats::get_sender();
                tx.send_async(buf.get_content()?).await?;
            }
            other => {
                godot_print!("未定义解析：{:?}", other);
            }
        };
        Ok(())
    }
}
