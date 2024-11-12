use std::collections::HashMap;

use crate::{host::Host, wol::Wake};

#[derive(Debug, Clone, serde::Serialize)]
pub struct Group {
    pub hosts: HashMap<String, Host>,
}

impl Wake for Group {
    fn wake(&self) -> () {
        for host in self.hosts.values() {
            host.wake();
        }
    }
}
