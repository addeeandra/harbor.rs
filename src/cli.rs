use std::collections::HashMap;
use r3bl_rs_utils_core::ch;
use r3bl_rs_utils_core::ChUnit;
use r3bl_tuify::{get_size, select_from_list, SelectionMode, StyleSheet};
use harbor_rs::compose::generate_compose;
use harbor_rs::writer::write_to_file;

fn main() {
    let compose_options = compose_options();
    let user_input = read_user_input(compose_options.clone());

    match user_input {
        Some(input) => {
            println!("Compose selection: {:?}", input);

            let selected = input.get(0).unwrap();
            generate(compose_options.get(selected).unwrap().iter().map(|it| it.to_string()).collect());

            println!("Generated docker-compose.yml file.");
        }
        None => println!("Aborting."),
    }
}

fn compose_options() -> HashMap<String, [&'static str; 3]> {
    HashMap::from([
        (String::from("mysql + redis (+ mailpit)"), ["mysql", "redis", "mailpit"]),
        (String::from("mariadb + redis (+ mailpit)"), ["mariadb", "redis", "mailpit"]),
        (String::from("pgsql + redis (+ mailpit)"), ["pgsql", "redis", "mailpit"]),
    ])
}

fn read_user_input(options: HashMap<String, [&'static str; 3]>) -> Option<Vec<String>> {
    select_from_list(
        "Select an item".to_string(),
        options.keys().map(|it| it.to_string().to_string()).collect(),
        options.len(),
        get_size().map(|it| it.col_count).unwrap_or(ch!(80)).into(),
        SelectionMode::Single,
        StyleSheet::default(),
    )
}

fn generate(service_names: Vec<String>) {
    let compose = generate_compose(service_names);
    write_to_file("./harbor_generated", "docker-compose.yml", compose);
}
