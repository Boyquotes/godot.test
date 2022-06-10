use super::*;
use model::{action,cmsg,stats};

lazy_static! {
    // upd 获取公网IP地址
    static ref MY_IPA:Arc<RwLock<String>> = {
        let s = ipadd::Conf::local_server();
        Arc::new(RwLock::new(s))
    };

    // upd 获取其他玩家IP地址
    static ref OTHERS_IPA:Arc<RwLock<Vec<String>>> = {
        let ip_list = Vec::new();
        Arc::new(RwLock::new(ip_list))
    };

    // udp 发送通道

    static ref S_CMSG:(Sender<cmsg::CMsg>, Receiver<cmsg::CMsg>) = unbounded();

    // udp 接收通道
    static ref R_CSTATS:(Sender<stats::Stats>, Receiver<stats::Stats>) = unbounded();
    static ref R_CACTION:(Sender<action::Action>, Receiver<action::Action>) = unbounded();

}

/**
 * 公网IP地址
 */
pub struct MyIPA;
impl MyIPA {
    pub fn get()->String{
        MY_IPA.read().to_string()
    }

    pub fn set(ipa: String){
        let mut ipal = MY_IPA.write();
        *ipal = ipa;
    }
}

/**
 * 其他玩家公网IP地址
 */
pub struct OthersIPA;
impl OthersIPA {
    pub fn get()->Vec<String>{
        OTHERS_IPA.read().to_vec()
    }

    pub fn set(ipa_list: Vec<String>){
        let mut ipal = OTHERS_IPA.write();
        *ipal = ipa_list;
    }
}

/**
 * 发送队列（Msg）
 */
pub struct SqMsg;
impl SqMsg {
    /**
     * 获取发送器
     */
    pub fn get_sender()->Sender<cmsg::CMsg>{
        S_CMSG.0.clone()
    }
    /**
     * 获取接收器
     */
    pub fn get_receiver()->Receiver<cmsg::CMsg>{
        S_CMSG.1.clone()
    }
}

/**
 * 接收队列（Stats）
 */
pub struct RqStats;
impl RqStats {
    /**
     * 获取发送器
     */
    pub fn get_sender()->Sender<stats::Stats>{
        R_CSTATS.0.clone()
    }
    /**
     * 获取接收器
     */
    pub fn get_receiver()->Receiver<stats::Stats>{
        R_CSTATS.1.clone()
    }
}

/**
 * 接收队列（Action）
 */
pub struct RqAction;
impl RqAction {
    /**
     * 获取发送器
     */
    pub fn get_sender()->Sender<action::Action>{
        R_CACTION.0.clone()
    }
    /**
     * 获取接收器
     */
    pub fn get_receiver()->Receiver<action::Action>{
        R_CACTION.1.clone()
    }
}