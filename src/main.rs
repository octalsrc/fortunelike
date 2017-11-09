extern crate getopts;
use getopts::Options;

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

fn user_db_default() -> Option<PathBuf> {
    std::env::var("HOME").ok()
        .map(|home: String|
             [home.as_str(), ".config", DB_DEFAULT]
             .iter()
             .collect())
}

fn sys_db_default() -> PathBuf {
    ["/", "etc", DB_DEFAULT].iter().collect()
}

fn get_file(path: PathBuf) -> Option<File> {
    File::open(path).ok()
}

fn choose_config(cmd_opt: Option<String>) -> Option<File> {
    let sysdb: Option<File> = get_file(sys_db_default());
    let userdb: Option<File> = user_db_default().and_then(get_file);
    let envdb: Option<File> =
        std::env::var(DB_VAR).ok()
        .map(PathBuf::from)
        .and_then(get_file);
    let cmddb: Option<File> =
        cmd_opt
        .map(PathBuf::from)
        .and_then(get_file);
    cmddb.or(envdb.or(userdb.or(sysdb)))
}

fn read_config(mut file: File) -> std::io::Result<Vec<String>> {
    let mut config_contents = String::new();
    file.read_to_string(&mut config_contents)?;
    let config_yaml = parse_yaml(&config_contents)?;
    get_values(config_yaml)
}    

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("f", "dbfile", "set database file path", "DB_FILE");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("i", "inline", "don't add tailing newline");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let dbfile = matches.opt_str("f");
    
    match choose_config(dbfile).and_then(|f| read_config(f).ok()) {
        Some(prompts) => {
            let mut rng = rand::thread_rng();
            let key = Range::new(0,prompts.len()).ind_sample(&mut rng);
            if matches.opt_present("i") {
                print!("{}",prompts[key])
            } else {
                println!("{}",prompts[key])
            }
        }
        None => println!("{}","[?]")
    }
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
