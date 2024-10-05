use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct DockerCompose {
    services: std::collections::HashMap<String, Service>,
    networks: std::collections::HashMap<String, Network>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Service {
    image: String,
    ports: Option<Vec<String>>,
    environment: Option<std::collections::HashMap<String, String>>,
    networks: Option<Vec<String>>,
    volumes: Option<Vec<String>>,
    healthcheck: HealthCheck,
}

#[derive(Debug, Deserialize, Serialize)]
struct HealthCheck {
    test: Option<Vec<String>>,
    retries: Option<u8>,
    timeout: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Network {
    driver: Option<String>,
}

fn read_from_file(name: &str) -> String {
    let file_name = format!("stubs/{}.yaml", name);
    let mut file = File::open(&file_name).expect(&format!("Failed to open {} file", file_name));
    let mut content = String::new();

    file.read_to_string(&mut content).expect(&format!("Failed to read {} file", file_name));

    content
}

fn read_service_from_file(name: &str) -> Service {
    let content = read_from_file(name);
    let service: Service = serde_yaml::from_str(&content).expect("Failed to parse YAML");

    service
}

fn read_network_from_file(name: &str) -> Network {
    let content = read_from_file(name);
    let network: Network = serde_yaml::from_str(&content).expect("Failed to parse YAML");

    network
}

fn main() {
    let redis = String::from("redis");
    let mysql = String::from("mysql");

    let redis_service = read_service_from_file(&redis);
    let mysql_service = read_service_from_file(&mysql);

    let network = read_network_from_file(&String::from("_network"));

    let compose = DockerCompose {
        services: HashMap::from([
            (redis, redis_service),
            (mysql, mysql_service),
        ]),
        networks: HashMap::from([
            (String::from("harbor"), network)
        ]),
    };

    let compose_yaml = serde_yaml::to_string(&compose).expect("Failed to convert yaml to file");

    let mut compose_file = File::create("docker-compose.yml").expect("Failed to create docker-compose.yml file");

    compose_file
        .write_all(compose_yaml.as_bytes())
        .expect("Failed to write to docker-compose.yml file");
}
