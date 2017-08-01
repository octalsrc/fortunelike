extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};

extern crate rand;
use rand::distributions::{IndependentSample, Range};

use std::fs::File;
use std::path::PathBuf;
use std::io::{Error, ErrorKind};
use std::io::prelude::*;

const DB_DEFAULT: &str = "fortunelike-db";
const DB_VAR: &str = "FORTUNELIKE_DB";

fn main() {
    let config_file = decide_db_path();
    match load_config(config_file) {
        Ok(prompts) => {
            let mut rng = rand::thread_rng();
            let key = Range::new(0,prompts.len()).ind_sample(&mut rng);
            println!("{}",prompts[key]);
        }
        Err(_) => println!("{}","[?]")
    }
}

fn decide_db_path() -> PathBuf {
    let mut path = ["etc", DB_DEFAULT].iter().collect();
    match std::env::var("HOME") {
        Ok(home) => path = [&home, ".config", DB_DEFAULT].iter().collect(),
        Err(_) => ()
    }
    match std::env::var(DB_VAR) {
        Ok(custom) => {
            path = PathBuf::new();
            path.push(custom)
        },
        Err(_) => ()
    }
    
    let mut path_flag = false;
    for a in std::env::args() {
        match path_flag {
            true => {
                path_flag = false;
                path = PathBuf::new();
                path.push(a)
            },
            false => {
                match Some(&*a) {
                    Some("-f") => path_flag = true,
                    Some("--dbfile") => path_flag = true,
                    _ => ()
                }
            }
        }
    }

    path
}

fn load_config(path: PathBuf) -> std::io::Result<Vec<String>> {
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
