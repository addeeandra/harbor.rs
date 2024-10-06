use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::{reader};

#[derive(Debug, Deserialize, Serialize)]
pub struct DockerCompose {
    pub networks: std::collections::HashMap<String, Network>,
    pub services: std::collections::HashMap<String, Service>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<std::collections::HashMap<String, Option<String>>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Service {
    image: String,
    ports: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<std::collections::HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    networks: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    healthcheck: Option<HealthCheck>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HealthCheck {
    test: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    retries: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Network {
    #[serde(skip_serializing_if = "Option::is_none")]
    driver: Option<String>,
}

pub fn generate_compose(service_names: Vec<String>) -> DockerCompose {
    let mut service_map = HashMap::new();
    let mut volume_map = HashMap::new();

    for service_name in service_names {
        let service = reader::read_service_from_stub(&service_name);
        service_map.insert(service_name, service.clone());

        if let Some(volume) = service.volumes {
            for volume_name in volume {
                let volume_key = volume_name.split(":").collect::<Vec<&str>>()[0].to_string();
                volume_map.insert(volume_key, None);
            }
        }
    }

    let network = reader::read_network_from_stub(&String::from("_network"));

    DockerCompose {
        services: service_map,
        networks: HashMap::from([
            (String::from("harbor"), network)
        ]),
        volumes: Some(volume_map),
    }
}