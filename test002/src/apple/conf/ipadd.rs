use super::*;



pub struct Conf{
    base:PathBuf
}
impl Conf {
    fn new() -> Self { 
        Self { base:PathBuf::from(Path::new(".")) } 
    }

    pub fn remote_server() -> String {
        let Self{ref base} = Self::new();
        let file = base.join("conf").join("ipadd.ini");
        let mut config = Ini::new();
        config.load(file).unwrap();
        let host = config.get("remote_server", "host").unwrap_or("127.0.0.1".to_owned());
        let port = config.get("remote_server", "port").unwrap_or("0".to_owned());
        format!("{}:{}", host, port)
    }

    pub fn local_server() -> String {
        let Self{ref base} = Self::new();
        let file = base.join("conf").join("ipadd.ini");
        let mut config = Ini::new();
        config.load(file).unwrap();
        let host = config.get("local_server", "host").unwrap_or("127.0.0.1".to_owned());
        let port = config.get("local_server", "port").unwrap_or("0".to_owned());
        format!("{}:{}", host, port)
    }


}
