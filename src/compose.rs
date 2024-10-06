use serde::{Deserialize, Serialize};

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