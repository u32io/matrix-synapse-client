use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;

pub fn read_file_as_unchecked<T: DeserializeOwned>(path: &str) -> T {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}