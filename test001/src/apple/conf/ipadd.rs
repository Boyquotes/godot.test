use std::path::{PathBuf,Path};
use configparser::ini::Ini;


pub struct URL;
impl URL {

    pub fn remote_server() -> String {
        let mut config = Ini::new();
        config.load(PathBuf::from(Path::new(".")).join("conf").join("ipadd.ini")).unwrap();
        let host = config.get("remote_server", "host").unwrap_or("127.0.0.1".to_owned());
        let port = config.get("remote_server", "port").unwrap_or("0".to_owned());
        format!("{}:{}", host, port)

    }

    pub fn local_server() -> String {
        let mut config = Ini::new();
        config.load(PathBuf::from(Path::new(".")).join("conf").join("ipadd.ini")).unwrap();
        let host = config.get("local_server", "host").unwrap_or("127.0.0.1".to_owned());
        let port = config.get("local_server", "port").unwrap_or("0".to_owned());
        format!("{}:{}", host, port)
    }


}
