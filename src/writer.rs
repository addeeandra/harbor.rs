use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::compose::DockerCompose;

pub fn write_to_file(generated_folder: &str, file_name: &str, compose: DockerCompose) {
    let compose_yaml = serde_yaml::to_string(&compose)
        .expect("Failed to convert yaml to file");

    let generated_path = Path::new(generated_folder);

    if generated_path.exists() {
        fs::remove_dir_all(generated_path)
            .expect("Failed to remove harbor-build directory");
    }

    fs::create_dir(generated_path).expect("Failed to create harbor-build directory");

    File::create(format!("{}/{}", generated_folder, file_name))
        .expect("Failed to create docker-compose.yml file")
        .write_all(compose_yaml.as_bytes())
        .expect("Failed to write to docker-compose.yml file");

    fs::copy("./stubs/_env.stub", format!("{}/{}", generated_folder, ".env"))
        .expect("Failed to create .env file");
}