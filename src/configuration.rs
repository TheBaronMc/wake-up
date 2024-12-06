use std::collections::HashMap;
use std::sync::RwLock;

use crate::group::Group;
use crate::host::Host;

type GroupList = HashMap<String, Group>;
type HostList = HashMap<String, Host>;

static GLOBAL_CONFIGURATION: RwLock<Option<Configuration>> = RwLock::new(None);

#[derive(Debug, Clone)]
pub struct Configuration {
    password: String,
    port: u16,
    api_enabled: bool,
    web_enabled: bool,
    groups: Option<GroupList>,
    hosts: Option<HostList>,
}

impl Configuration {
    pub fn new(
        password: String,
        port: u16,
        api_enabled: bool,
        web_enabled: bool,
        groups: Option<GroupList>,
        hosts: Option<HostList>,
    ) -> Self {
        Configuration {
            password,
            port,
            api_enabled,
            web_enabled,
            groups,
            hosts,
        }
    }

    fn update(&mut self, new_configuration: &Configuration) -> &Self {
        self.password = new_configuration.password.clone();
        self.groups = new_configuration.groups.clone();
        self.hosts = new_configuration.hosts.clone();

        self
    }

    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn port(&self) -> &u16 {
        &self.port
    }

    pub fn api_enabled(&self) -> &bool {
        &self.api_enabled
    }

    pub fn web_enabled(&self) -> &bool {
        &self.web_enabled
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
            api_enabled: true,
            web_enabled: true,
            groups: None,
            hosts: None,
        }
    }
}

pub fn read_global_configuration<F, R>(f: F) -> R
where
    F: FnOnce(Option<&Configuration>) -> R,
{
    let guard = GLOBAL_CONFIGURATION.read().unwrap();
    f((*guard).as_ref())
}

pub fn update_global_configuration(configuration: &Configuration) {
    let mut write_lock = GLOBAL_CONFIGURATION.write().unwrap();

    if let None = *write_lock {
        *write_lock = Some(configuration.clone());
    } else {
        let actual_config = (*write_lock).as_mut().unwrap();
        actual_config.update(configuration);
    }
}
