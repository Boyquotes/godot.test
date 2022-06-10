use super::*;



#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub account: String,
    pub password: String,
}

impl LoginUser {
    pub fn login(&self) {
        let client = reqwest::blocking::Client::new();
        let res = client
            .post("http://118.193.46.124:5002/login")
            .json(&self)
            .send()
            .unwrap();
        godot_print!("{:?}", res.status());
        godot_print!("{:?}", res.text());
    }
}



