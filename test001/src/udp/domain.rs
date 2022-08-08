use super::*;
use tokio::time::{sleep, Duration};
use chashmap::CHashMap;
use spin::RwLock;
use std::sync::Arc;


lazy_static! {
    static ref HOST: Arc<RwLock<Option<NetIP>>> = Arc::new(RwLock::new(None));
    static ref CACHE:CHashMap<NetIP, Sign> = Default::default();
}

pub struct Cursor;

impl Cursor {
    pub fn _get(key:NetIP)->Option<Sign>{
        let sign = CACHE.get(&key)?;
        Some(sign.to_owned())
    }
    pub fn exist(key:&NetIP)-> bool{
        CACHE.contains_key(key)
    }
    pub fn find()->Domain{
        let data = CACHE.clone();
        Domain{data}
    }
    pub fn replace_one(ipm:IpMap)->Option<Sign>{
        CACHE.insert(ipm.ipa,ipm.sign)
    }

    pub fn get_host()->Option<NetIP>{
        let host = HOST.read();
        host.clone()
    }

    pub fn set_host(host:NetIP){
        let mut whost = HOST.write();
        *whost = Some(host);
    }

}



/** 
 * 网络映射
 */
#[derive(Debug,Clone)]
pub struct Domain {
    pub data: CHashMap<NetIP,Sign>
}
impl Domain {
    pub fn _new() -> Self {
        let data = CHashMap::new();
        Self {data}
    }

    /**
     * IP探测
     */
    pub async fn start()-> Result<()> {
        for (ipa,sign) in Cursor::find().data {
            let ipm = IpMap::new(ipa,sign);
            Self::ask(ipm).await?;      
        }
        sleep(Duration::from_secs(1)).await;
        Ok(())
    }

    

    async fn ask(ipm:IpMap)->Result<()>{
        match ipm.sign {
            Sign::Ready=>{
                let mut msg = Msg::new(ipm.ipa.ip.clone(), ipm.ipa.port, "P2P-ASK".to_owned());
                if let Some(ipn) = Cursor::get_host(){
                    msg.set_object(ipn)?;
                    let buf = msg.to_buf()?;
                    Launch::ready_async(buf).await?;
                    Cursor::replace_one(ipm.wait());
                }
            }
            Sign::Wait(n)=>{
                if n >= 60{
                    Cursor::replace_one(ipm.ready());
                }else{
                    Cursor::replace_one(ipm.wait());
                    
                }
            }
            _=>{}
        };
        
        Ok(())
    }

    /**
    * 收到数据并回复
    * msg 接收到的消息
    */ 
    pub async fn accept(msg:Msg)->Result<Msg>{
        // 接收
        let nip:NetIP = msg.get_object()?;
        let ipm = IpMap::new(nip,Sign::Rigth(msg.port));
        Cursor::replace_one(ipm);
        // 回复
        let mut msg = Msg::new(msg.ip, msg.port, "P2P-RSP".to_owned());
        if let Some(ip) = Cursor::get_host(){
            msg.set_object(ip)?;
            let buf = msg.to_buf()?;
            Launch::ready_async(buf).await?;
        }
        Ok(msg)
    }

    /**
    * 得到回复
    * msg 接收到的消息
    */ 
    pub fn rsp(msg:Msg)->Result<()>{
        let ipn:NetIP = msg.get_object()?;
        let ipm = IpMap::new(ipn,Sign::Rigth(msg.port));
        Cursor::replace_one(ipm); 
        Ok(())
    }

}
