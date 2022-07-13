use crate::apple::Result;
use flume::{unbounded, Receiver, Sender};
use super::data::Buf;

lazy_static! {
    static ref SEND_CHANNEL: (Sender<Buf>, Receiver<Buf>) = unbounded();
    static ref RECV_CHANNEL: (Sender<Buf>, Receiver<Buf>) = unbounded();
}

/**
 * 发送通道
 */
pub struct Launch;
impl Launch {

    pub fn ready(buf:Buf)->Result<()>{
        let a = SEND_CHANNEL.0.clone();
        a.send(buf)?;
        Ok(())
    }

    pub async fn ready_async(buf:Buf)->Result<()>{
        let a = SEND_CHANNEL.0.clone();
        a.send_async(buf).await?;
        Ok(())
    }

    pub fn go()->Result<Buf>{
        let a = SEND_CHANNEL.1.clone();
        let buf = a.recv()?;
        Ok(buf)
    }

    pub async fn go_async()->Result<Buf>{
        let a = SEND_CHANNEL.1.clone();
        let buf = a.recv_async().await?;
        Ok(buf)
    }

}
/**
 * 接收通道
 */
pub struct Accept;
impl Accept {

    pub fn put(buf:Buf)->Result<()>{
        let a = RECV_CHANNEL.0.clone();
        a.send(buf)?;
        Ok(())
    }

    pub async fn put_async(buf:Buf)->Result<()>{
        let a = RECV_CHANNEL.0.clone();
        a.send_async(buf).await?;
        Ok(())
    }

    pub fn get()->Result<Buf>{
        let a = RECV_CHANNEL.1.clone();
        let buf = a.recv()?;
        Ok(buf)
    }

    pub async fn get_async()->Result<Buf>{
        let a = RECV_CHANNEL.1.clone();
        let buf = a.recv_async().await?;
        Ok(buf)
    }
}
