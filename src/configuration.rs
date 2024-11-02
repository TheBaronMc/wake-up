use std::collections::HashMap;
use std::sync::RwLock;
use std::fs;

use yaml_rust::{Yaml, YamlLoader};

use crate::host::Host;
use crate::group::Group;

#[derive(Debug)]
pub struct Configuration {
    pub password: String,
    pub groups: Option<HashMap<String, Group>>,
    pub hosts: Option<HashMap<String,Host>>
}

pub static CONFIGURATION_PATH: &str = "configuration.yml";

pub static CONFIGURATION: RwLock<Configuration> = RwLock::new(Configuration{
    password: String::new(),
    groups: None,
    hosts: None
});

pub fn read_configuration(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Ok(file_content) => {
            match  YamlLoader::load_from_str(file_content.as_str()) {
                Ok(documents) => {
                    let new_configuration = parse_configuration( &documents[0])?;
                    update_configuation(new_configuration);
                    Ok(())
                },
                Err(error) => {
                    Err(error.to_string())
                }
            }
        },
        Err(error) => { Err(error.to_string()) }
    }
}

fn update_configuation(new_configuration: Configuration) {
    match CONFIGURATION.write() {
        Ok(mut lock_guard) => {
            *lock_guard = new_configuration;
        },
        Err(_) => eprintln!("Failed to update app configuration")
    }
}

fn parse_configuration(document: &Yaml) -> Result<Configuration, String> {
    Ok(Configuration {
        password: String::from(document["password"].as_str().expect("No password defined")),
        groups:  parse_groups(&document["groups"])?, //document["groups"].as_vec().map(f),
        hosts: parse_hosts(&document["hosts"])?
    })
}

fn parse_groups(document: &Yaml) -> Result<Option<HashMap<String, Group>>, String> {
    match document.as_hash() {
        Some(hash) => {
            let mut groups: HashMap<String, Group> = HashMap::new();

            for entry in hash {
                let name: String = String::from(entry.0.as_str().expect("Failed to parse group name"));
                let group = parse_group(entry.1)?;
                groups.insert(name, group);
            }

           Ok(Some(groups))
        },
        None => Ok(None)
    }
}

fn parse_group(document: &Yaml) -> Result<Group, String> {
    match parse_hosts(document)? {
        None => Err(String::from("No hosts defined")),
        Some(hosts) => Ok(Group {hosts }),
    }
}

fn parse_hosts(document: &Yaml) -> Result<Option<HashMap<String, Host>>, String> {
    match document.as_hash() {
        Some(hash) => {
            let mut hosts: HashMap<String, Host> = HashMap::new();

            for entry in hash {
                let name: String = String::from(entry.0.as_str().expect("Failed to parse host name"));
                let host = parse_host(entry.1)?;
                hosts.insert(name, host);
            }

           Ok(Some(hosts))
        },
        None => Ok(None)
    }
}


fn parse_host(document: &Yaml) -> Result<Host, String> {
    let mut port: u16 = 6;
    if let Some(p ) = document["port"].as_i64() {
        port = p as u16;
    }

    match document["address"].as_str() {
        Some(address) => {
            Ok(Host {
                port,
                address: parse_address(address)
            }) 
        },
        None => Err(String::from("Missing adress"))
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