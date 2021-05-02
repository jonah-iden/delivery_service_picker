extern crate json;
use json::{parse};

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub struct DelService {
    pub name: String,
    pub url: String,
    pub weight: i32,
}

impl DelService {

}

pub fn read_services_from_json(file_path: &str) -> Vec<DelService>{         
     match read_json_from_file(file_path) {
        Ok(data) => {
            let parsed = json::parse(&data).unwrap();
            let mut services: Vec<DelService> = Vec::new();
            for service in parsed["services"].members() {
                services.push(DelService {
                    name: String::from(service["name"].as_str().unwrap()),
                    url: String::from(service["url"].as_str().unwrap()),
                    weight: 1
                });
            }
            return services;
        },
        Err(error) => {
            println!("error while reading file: {}", error);
            return Vec::new();
        }
    }
}

fn read_json_from_file(file_path: &str) -> io::Result<String>{
    let f = File::open(file_path)?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    reader.read_to_string(&mut buffer)?; 
    return Ok(buffer)
}