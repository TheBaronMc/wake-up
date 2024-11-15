use std::collections::HashMap;
use std::sync::RwLock;

use crate::group::Group;
use crate::host::Host;

type GroupList = HashMap<String, Group>;
type HostList = HashMap<String, Host>;

static CONFIGURATION: RwLock<Option<Configuration>> = RwLock::new(None);

#[derive(Debug, Clone)]
pub struct Configuration {
    password: String,
    port: u16,
    groups: Option<GroupList>,
    hosts: Option<HostList>,
}

impl Configuration {
    pub fn new(
        password: String,
        port: u16,
        groups: Option<GroupList>,
        hosts: Option<HostList>,
    ) -> Self {
        Configuration {
            password,
            port,
            groups,
            hosts,
        }
    }

    fn update(&mut self, new_configuration: &Configuration) {
        self.password = new_configuration.password.clone();
        self.groups = new_configuration.groups.clone();
        self.hosts = new_configuration.hosts.clone();
    }

    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn port(&self) -> &u16 {
        &self.port
    }

    pub fn groups(&self) -> &Option<GroupList> {
        &self.groups
    }

    pub fn hosts(&self) -> &Option<HostList> {
        &self.hosts
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            password: String::from("wake-up!"),
            port: 8999,
            groups: None,
            hosts: None,
        }
    }
}

pub fn read_configuration() -> Option<Configuration> {
    CONFIGURATION.read().unwrap().clone()
}

pub fn write_configuration(configuration: Configuration) -> Configuration {
    let mut write_lock = CONFIGURATION.write().unwrap();

    if let None = *write_lock {
        *write_lock = Some(configuration.clone());
    } else {
        let actual_config = (*write_lock).as_mut().unwrap();
        actual_config.update(&configuration);
    }

    configuration
}
