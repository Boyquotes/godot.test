use super::{Buf, ChannelR, ChannelS, Msg};
use crate::apple::conf::ipadd::URL;
use crate::apple::Result;
use std::{net::SocketAddr, str::FromStr, sync::Arc};
use tokio::net::UdpSocket;

/**UdpServer
 * UDP发送和接收服务
 * fn new() 创建socket任务（Task）
 * pub async fn start(&self)  启动服务
 * fn udp_sender(&self) 从SQueue队列通道接收数据，socket发送数据
 * fn udp_accept(&self) 从socket接收数据，转存数据RQueue队列通道
 */
#[derive(Debug, Clone)]
pub struct Task {
    sock: Arc<UdpSocket>,
}
impl Task {
    pub async fn new() -> Result<Self> {
        let add = URL::local_server();
        let add = SocketAddr::from_str(&add)?;
        let sock = UdpSocket::bind(add).await?;
        Ok(Self {
            sock: Arc::new(sock),
        })
    }

    pub async fn udp_sender(&self) -> Result<Msg> {
        let send_buf = ChannelS::get().recv_async().await?;
        self.sock
            .send_to(&send_buf.bytes, send_buf.get_target())
            .await?;
        Ok(send_buf.to_msg()?)
    }

    pub async fn udp_accept(&self) -> Result<Msg> {
        let mut recv_buf = [0; 1024];
        let (len, addr) = self.sock.recv_from(&mut recv_buf).await?;
        let ip = addr.ip().to_string();
        let port = addr.port();
        let bytes = recv_buf[..len].to_vec();
        let buf = Buf::new(ip, port, bytes);
        ChannelR::set().send_async(buf.clone()).await?;
        let msg = buf.to_msg()?;
        Ok(msg)
    }
}
