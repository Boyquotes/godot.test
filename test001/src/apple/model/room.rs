use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ip {
    pub ip: String,
    pub time: SystemTime,
}

impl Ip {
    pub fn new(ip: String) -> Self {
        Self {
            ip,
            time: SystemTime::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Room {
    pub key: i32,
    pub ip_list: Vec<Ip>,
}

impl Room {
    pub fn ip(&self) -> Vec<String> {
        let mut ip_list = Vec::<String>::new();
        for i in &self.ip_list {
            ip_list.push(i.ip.clone())
        }
        ip_list
    }
}
