use crate::wol::{wake_on_lan, Wake};

#[derive(Debug, serde::Serialize)]
pub struct Host {
    pub address: [u8; 12],
    pub port: u16,
}

impl Wake for Host {
    fn wake(&self) -> () {
        wake_on_lan(&self.address, &self.port);
    }
}
