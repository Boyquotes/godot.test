use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Clone)]
pub struct NetIP {
    pub ip: String,
    pub port: u16,
}
impl NetIP {
    pub fn new(ip: String, port: u16) -> Self { Self { ip, port } }
}

/**
 * 
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Sign {
    Ready,
    Wait(u16),
    Rigth(u16),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IpMap{
    pub ipa:NetIP,
    pub sign:Sign,
}

impl IpMap {
    pub fn new(ipa:NetIP, sign:Sign) -> Self { 
        Self { ipa, sign} 
    }
    pub fn ready(&self)->Self{
        Self {ipa:self.ipa.clone(), sign:Sign::Ready} 
    }
    pub fn wait(&self)->Self{
        match self.sign{
            Sign::Ready=>{
                Self {ipa:self.ipa.clone(), sign:Sign::Wait(0)} 
            }
            Sign::Wait(s) => {
                Self {ipa:self.ipa.clone(), sign:Sign::Wait(s+1)} 
            }
            Sign::Rigth(_) => {
                self.clone()
            },
        }
    }

    pub fn rigth(&self,port:u16)->Self{
        Self{ipa:self.ipa.clone(), sign:Sign::Rigth(port)}
    }
}
