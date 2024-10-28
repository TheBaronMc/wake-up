use crate::wol::{wake_on_lan, Wake};

#[derive(Debug)]
#[derive(serde::Serialize)]
pub struct Host {
    pub address: String,
    pub port: u16
}

impl Wake for Host {
    fn wake(&self) -> () {
        let address: &str = &self.address.as_str();
        let port: &u16 = &self.port;
        print!("Sending magic packet to host: {}, port: {} ->", address, port);
        wake_on_lan(address, port);
        println!("Ok");
    }
}