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



