use crate::wol::{wake_on_lan, Wake};

#[derive(Debug)]
#[derive(serde::Serialize)]
pub struct Host {
    pub address: [u8; 12],
    pub port: u16
}

impl Wake for Host {
    fn wake(&self) -> () {
        let address: &[u8; 12] = &self.address;
        let address_str = address.iter().map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()    
        .join(":");
        let port: &u16 = &self.port;
        print!("Sending magic packet to host: {}, port: {} ->", address_str, port);
        wake_on_lan(address, port);
        println!("Ok");
    }
}