use std::{fs, path::Path};

pub fn load_emails(f_name: &str) -> Vec<String> {
    if !Path::new(f_name).exists() {
        fs::File::create(f_name).unwrap();
        println!("Created file: {}", f_name);
    };

    let file = fs::read_to_string(f_name).unwrap();

    let strings = file
        .split("\n")
        .into_iter()
        .map(|s| s.to_owned().replace("\r", ""))
        .collect::<Vec<String>>();

    strings
}
