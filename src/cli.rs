use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Write};
use std::path::Path;
use r3bl_rs_utils_core::ch;
use r3bl_rs_utils_core::ChUnit;
use r3bl_tuify::{get_size, select_from_list, SelectionMode, StyleSheet};
use harbor_rs::reader;
use harbor_rs::compose;

fn main() {
    let compose_options = HashMap::from([
        (String::from("mysql + redis (+ mailpit)"), ["mysql", "redis", "mailpit"]),
        (String::from("mariadb + redis (+ mailpit)"), ["mariadb", "redis", "mailpit"]),
        (String::from("pgsql + redis (+ mailpit)"), ["pgsql", "redis", "mailpit"]),
    ]);

    let user_input = select_from_list(
        "Select an item".to_string(),
        compose_options.keys().map(|it| it.to_string().to_string()).collect(),
        compose_options.len(),
        get_size().map(|it| it.col_count).unwrap_or(ch!(80)).into(),
        SelectionMode::Single,
        StyleSheet::default(),
    );

    match &user_input {
        Some(it) => {
            println!("User selected: {:?}", it);

            let selected = it.get(0).unwrap();
            generate_compose(compose_options.get(selected).unwrap().iter().map(|it| it.to_string()).collect());
        }
        None => println!("User did not select anything"),
    }
}

fn generate_compose(service_names: Vec<String>) {
    let mut service_map = HashMap::new();
    let mut volume_map = HashMap::new();

    for service_name in service_names {
        let service = reader::read_service_from_file(&service_name);
        service_map.insert(service_name, service.clone());

        if let Some(volume) = service.volumes {
            for volume_name in volume {
                let volume_key = volume_name.split(":").collect::<Vec<&str>>()[0].to_string();
                volume_map.insert(volume_key, None);
            }
        }
    }

    let network = reader::read_network_from_file(&String::from("_network"));

    let compose = compose::DockerCompose {
        services: service_map,
        networks: HashMap::from([
            (String::from("harbor"), network)
        ]),
        volumes: Some(volume_map),
    };

    let compose_yaml = serde_yaml::to_string(&compose).expect("Failed to convert yaml to file");

    if Path::new("./harbor_generated").exists() {
        fs::remove_dir_all("./harbor_generated").expect("Failed to remove harbor-build directory");
    }

    fs::create_dir("./harbor_generated").expect("Failed to create harbor-build directory");
    fs::copy("./stubs/_env.stub", "../harbor_generated/.env").expect("Failed to create .env file");

    let mut compose_file = File::create("./harbor_generated/docker-compose.yml").expect("Failed to create docker-compose.yml file");

    compose_file
        .write_all(compose_yaml.as_bytes())
        .expect("Failed to write to docker-compose.yml file");
}
