use crate::apple::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use md5;



/**
 * 数据编辑
 */
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Msg {
    pub ip: String,
    pub port: u16,
    pub data: HashMap<String, String>,
}
impl Msg {
    pub fn new(ip: String, port: u16, type1: String) -> Self {
        let mut map = HashMap::new();
        map.insert("type".to_owned(), type1);
        Self {
            ip,
            port,
            data: map,
        }
    }

    pub fn insert(&mut self,key:String,value:String)->Option<String>{
        self.data.insert(key, value)
    }

    pub fn set_object<T: Serialize>(&mut self, obj: T) -> Result<()> {
        let object = serde_json::to_string(&obj)?;
        self.data.insert("object".to_owned(), object);
        Ok(())
    }
    pub fn to_buf(&self) -> Result<Buf> {
        let bytes = serde_json::to_vec(&self.data)?;
        Ok(Buf {
            ip: self.ip.clone(),
            port: self.port,
            bytes,
        })
    }
    pub fn get_type(&self) -> Option<String> {
        let type1 = self.data.get("type")?.to_owned();
        Some(type1)
    }
    pub fn get_object<'a, T>(&'a self) -> Result<T>
    where
        T: Deserialize<'a>,
    {
        let a = self.data.get("object").unwrap();
        let object = serde_json::from_str(a).unwrap();
        Ok(object)
    }
}

/**
 * 数据发送
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Buf {
    pub ip: String,
    pub port: u16,
    pub bytes: Vec<u8>,
}
impl Buf {
    pub fn new(ip: String, port: u16, bytes: Vec<u8>) -> Self {
        Self { ip, port, bytes }
    }
    pub fn to_msg(&self) -> Result<Msg> {
        let data: HashMap<String, String> = serde_json::from_slice(&self.bytes)?;
        Ok(Msg {
            ip: self.ip.clone(),
            port: self.port,
            data,
        })
    }
    pub fn get_target(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    pub fn get_md5(&self)-> String {
        let digest = md5::compute(self.bytes.clone());
        format!("{:?}",digest)
    }
}
