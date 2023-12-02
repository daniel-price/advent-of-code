pub use std::fs::{self};

pub fn input(suffix: &str, file_name: &str) -> String {
    let day = file_name.replace("src/", "").replace(".rs", "");
    let file_path = "../inputs/".to_owned() + &day + "-" + suffix + ".txt";

    fs::read_to_string(file_path.clone())
        .unwrap_or_else(|_| panic!("Could not read file {}", file_path))
        .trim()
        .to_string()
}
