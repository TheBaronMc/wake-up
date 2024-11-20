use std::collections::HashMap;
use std::{env, fs};

use yaml_rust::{Yaml, YamlLoader};

use crate::configuration::{read_configuration, write_configuration, Configuration};
use crate::group::Group;
use crate::host::Host;

pub static PORT_ENV_VAR: &str = "WAKE_UP_PORT";
pub static PASSWORD_ENV_VAR: &str = "WAKE_UP_PASSWORD";
pub static API_ENABLED_ENV_VAR: &str = "WAKE_UP_API_ENABLED";
pub static WEB_ENABLED_ENV_VAR: &str = "WAKE_UP_WEB_ENABLED";

#[derive(Debug)]
struct ConfigurationFromFile {
    password: Option<String>,
    port: Option<u16>,
    api_enabled: Option<bool>,
    web_enabled: Option<bool>,
    groups: Option<HashMap<String, Group>>,
    hosts: Option<HashMap<String, Host>>,
}

pub fn load_configuration() -> Result<Configuration, String> {
    debug!("[CONFIGURATION] loading configuration");

    let current_configuration = read_configuration().unwrap_or(Configuration::default());
    debug!(
        "[CONFIGURATION] Current configuration {:?}",
        current_configuration
    );

    let configuration_from_file = read_configuration_file("configuration.yml")?;
    debug!(
        "[CONFIGURATION] Configuration from file {:?}",
        configuration_from_file
    );

    let new_password: String = env::var(PASSWORD_ENV_VAR).ok().unwrap_or(
        configuration_from_file
            .password
            .unwrap_or(current_configuration.password().clone()),
    );

    let new_port: u16 = match env::var(PORT_ENV_VAR).ok() {
        Some(value) => {
            u16::from_str_radix(value.as_str(), 10).unwrap_or(current_configuration.port().clone())
        }
        None => configuration_from_file
            .port
            .unwrap_or(current_configuration.port().clone()),
    };

    let api_enabled: Option<bool> = env::var(API_ENABLED_ENV_VAR)
        .ok()
        .and_then(|s| str_to_bool(s.as_str()));

    let web_enabled: Option<bool> = env::var(WEB_ENABLED_ENV_VAR)
        .ok()
        .and_then(|s| str_to_bool(s.as_str()));

    let new_configuration: Configuration = Configuration::new(
        new_password,
        new_port,
        api_enabled.unwrap_or(
            configuration_from_file
                .api_enabled
                .unwrap_or(current_configuration.api_enabled().clone()),
        ),
        web_enabled.unwrap_or(
            configuration_from_file
                .web_enabled
                .unwrap_or(current_configuration.web_enabled().clone()),
        ),
        configuration_from_file.groups,
        configuration_from_file.hosts,
    );

    debug!("[configuration] New Configuration {:?}", new_configuration);
    Ok(write_configuration(new_configuration))
}

fn str_to_bool(s: &str) -> Option<bool> {
    match s {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn read_configuration_file(path: &str) -> Result<ConfigurationFromFile, String> {
    match fs::read_to_string(path) {
        Ok(file_content) => match YamlLoader::load_from_str(file_content.as_str()) {
            Ok(documents) => parse_configuration(&documents[0]),
            Err(error) => Err(error.to_string()),
        },
        Err(error) => Err(error.to_string()),
    }
}

fn parse_configuration(document: &Yaml) -> Result<ConfigurationFromFile, String> {
    Ok(ConfigurationFromFile {
        password: document["password"]
            .as_str()
            .and_then(|s| Some(String::from(s))),
        port: document["port"].as_i64().and_then(|p| Some(p as u16)),
        api_enabled: document["api_enabled"].as_bool(),
        web_enabled: document["web_enabled"].as_bool(),
        groups: parse_groups(&document["groups"])?, //document["groups"].as_vec().map(f),
        hosts: parse_hosts(&document["hosts"])?,
    })
}

fn parse_groups(document: &Yaml) -> Result<Option<HashMap<String, Group>>, String> {
    match document.as_hash() {
        Some(hash) => {
            let mut groups: HashMap<String, Group> = HashMap::new();

            for entry in hash {
                let name: String =
                    String::from(entry.0.as_str().expect("Failed to parse group name"));
                let group = parse_group(entry.1)?;
                groups.insert(name, group);
            }

            Ok(Some(groups))
        }
        None => Ok(None),
    }
}

fn parse_group(document: &Yaml) -> Result<Group, String> {
    match parse_hosts(document)? {
        None => Err(String::from("No hosts defined")),
        Some(hosts) => Ok(Group { hosts }),
    }
}

fn parse_hosts(document: &Yaml) -> Result<Option<HashMap<String, Host>>, String> {
    match document.as_hash() {
        Some(hash) => {
            let mut hosts: HashMap<String, Host> = HashMap::new();

            for entry in hash {
                let name: String =
                    String::from(entry.0.as_str().expect("Failed to parse host name"));
                let host = parse_host(entry.1)?;
                hosts.insert(name, host);
            }

            Ok(Some(hosts))
        }
        None => Ok(None),
    }
}

fn parse_host(document: &Yaml) -> Result<Host, String> {
    let mut port: u16 = 6;
    if let Some(p) = document["port"].as_i64() {
        port = p as u16;
    }

    match document["address"].as_str() {
        Some(address) => Ok(Host {
            port,
            address: parse_address(address),
        }),
        None => Err(String::from("Missing adress")),
    }
}

fn parse_address(address: &str) -> [u8; 12] {
    let mut bytes = [0u8; 12];

    for (i, part) in address.split(':').enumerate() {
        bytes[i] = u8::from_str_radix(part, 16)
            .expect("Each part of the address should be a valid hexadecimal number");
    }

    bytes
}
