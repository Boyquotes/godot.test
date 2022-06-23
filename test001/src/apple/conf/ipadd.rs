pub struct URL;
impl URL {
    pub fn remote_server() -> String {
        let host = "118.193.46.124";
        let port = 5016;
        format!("{}:{}", host, port)
    }

    pub fn local_server() -> String {
        let host = "0.0.0.0";
        let port = 5016;
        format!("{}:{}", host, port)
    }
}
