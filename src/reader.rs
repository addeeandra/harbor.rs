use std::fs::File;
use std::io::Read;
use crate::compose::{Network, Service};

pub fn read_from_stub(name: &str) -> String {
    let file_name = format!("./stubs/{}.stub", name);
    let mut file = File::open(&file_name).expect(&format!("Failed to open {} file", file_name));
    let mut content = String::new();

    file.read_to_string(&mut content).expect(&format!("Failed to read {} file", file_name));

    content
}

pub fn read_service_from_file(name: &str) -> Service {
    let content = read_from_stub(name);
    let service: Service = serde_yaml::from_str(&content).expect("Failed to parse YAML");

    service
}

pub fn read_network_from_file(name: &str) -> Network {
    let content = read_from_stub(name);
    let network: Network = serde_yaml::from_str(&content).expect("Failed to parse YAML");

    network
}