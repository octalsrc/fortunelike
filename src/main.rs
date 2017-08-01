
extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};

extern crate rand;
use rand::distributions::{IndependentSample, Range};

use std::fs::File;
use std::io::{Error, ErrorKind};
use std::io::prelude::*;

fn main() {
    match load_config("test.yaml") {
        Ok(prompts) => {
            let mut rng = rand::thread_rng();
            let key = Range::new(0,prompts.len()).ind_sample(&mut rng);
            println!("{}",prompts[key]);
        }
        Err(e) => println!("{}",e)
    }
}

fn load_config(path: &str) -> std::io::Result<Vec<String>> {
    let mut config_file = File::open(path)?;
    let mut config_contents = String::new();
    config_file.read_to_string(&mut config_contents)?;
    let config_yaml = parse_yaml(&config_contents)?;
    get_values(config_yaml)
}

fn parse_yaml(path: &str) -> std::io::Result<Yaml> {
    match YamlLoader::load_from_str(path) {
        Ok(mut yv) => match yv.pop() {
            Some(y) => Ok(y),
            None => Err(Error::new(ErrorKind::Other, "Empty Yaml doc."))
        },
        Err(_) => Err(Error::new(ErrorKind::Other, "Yaml parse failed."))
    }
}

fn get_values(yaml: Yaml) -> std::io::Result<Vec<String>> {
    let error = Err(Error::new(ErrorKind::Other, "Config file malformed."));
    match yaml.as_vec() {
        Some(yv) => Ok(yv.iter().fold(
            Vec::new(),
            |mut v,y| {
                v.push(y.as_str().unwrap().to_string());
                v
            })),
        None => error
    }
}