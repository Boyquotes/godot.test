use super::*;

/**
* udp发送数据管道封装
*/
#[derive(Debug, Clone)]
pub struct CMsg {
    pub buf: Buf,
    pub ipadd: String,
}

/**
 * udp发送数据类型
 */
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Class {
    IP,
    ROOM,
    STATS,
    ACTION,
}

/**
 * udp发送数据标题（头）
 */
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Head {
    pub cs: Class,
    pub seq: isize,
    pub next: isize,
}

/**
 * udp发送数据
 */
#[derive(Debug, Clone)]
pub struct Data<T> {
    head: Head,
    content: Option<T>,
}
impl<T> Data<T>
where
    T: Serialize,
{
    pub fn new(head: Head, content: Option<T>) -> Self {
        Self { head, content }
    }
    /**
     * 数据封装
     */
    pub fn make(&self) -> Result<Buf>
    where
        T: Serialize,
    {
        let mut buf = Buf::new();
        // 封装<-标题
        match serde_json::to_vec(&self.head) {
            Ok(list) => {
                for (i, v) in list.iter().enumerate() {
                    buf.bytes[i] = *v
                }
            }
            Err(err) => return Err(err.into()),
        };
        // 封装<-内容
        if let Some(content) = &self.content {
            match serde_json::to_vec(&content) {
                Ok(list) => {
                    for (i, v) in list.iter().enumerate() {
                        let j = i + 50;
                        buf.bytes[j] = *v
                    }
                }
                Err(err) => return Err(err.into()),
            };
        }
        Ok(buf)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Buf {
    pub bytes: [u8; 1024],
}
impl Buf {
    pub fn new() -> Self {
        Self {
            bytes: [0_u8; 1024],
        }
    }
    /**
     *提取->标题
     */
    pub fn get_head(&self) -> Result<Head> {
        let str = match str::from_utf8(&self.bytes[0..49]) {
            Ok(s) => s.trim_matches(char::from(0)),
            Err(err) => return Err(err.into()),
        };
     

        let obj = match serde_json::from_str(&str) {
            Ok(obj) => obj,
            Err(err) => return Err(err.into()),
        };
        Ok(obj)
    }
    /**
     * 提取->内容
     */
    pub fn get_content<'a, T>(&'a self) -> Result<T>
    where
        T: Deserialize<'a>,
    {
        let str = match str::from_utf8(&self.bytes[50..1024]) {
            Ok(s) => s.trim_matches(char::from(0)),
            Err(err) => return Err(err.into()),
        };
        let obj = match serde_json::from_str::<T>(&str) {
            Ok(s) => s,
            Err(err) => return Err(err.into()),
        };
        Ok(obj)
    }
}
