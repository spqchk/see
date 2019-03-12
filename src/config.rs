

extern crate yaml_rust;
use yaml_rust::{YamlLoader};
use std::fs;
use std::result::Result;


#[derive(Debug)]
pub struct ServerConfig {
    pub default: bool,
    pub host: String,
    pub listen: i64,
    pub root: String,
    pub log: Log
}


#[derive(Debug)]
pub struct Log {
    pub success: String,
    pub error: String
}


impl ServerConfig {

    pub fn new(path: &str) -> Result<Vec<Vec<ServerConfig>>, String>  {

        let str = fs::read_to_string(path).unwrap();

        let docs = YamlLoader::load_from_str(&str).unwrap();

        let mut configs: Vec<Vec<ServerConfig>> = vec![];

        let servers = &docs[0].as_vec().unwrap();

        for x in servers.iter() {

            let server = &x["server"];

            let default = match &server["default"].as_bool() {
                Some(d) => *d,
                None => false
            };

            let host = match &server["host"].as_str() {
                Some(d) => *d,
                None => ""
            }.to_string();

            let listen = match &server["listen"].as_i64() {
                Some(d) => *d,
                None => 0
            };

            let root = match &server["root"].as_str() {
                Some(d) => *d,
                None => ""
            }.to_string();

            let success = match &server["log"]["success"].as_str() {
                Some(d) => *d,
                None => ""
            }.to_string();

            let error = match &server["log"]["error"].as_str() {
                Some(d) => *d,
                None => ""
            }.to_string();

            let config = ServerConfig {
                default,
                host,
                listen,
                root,
                log: Log {
                    success,
                    error
                }
            };

            let mut has = false;
            let mut n = 0;
            for (i, items) in configs.iter().enumerate() {
                if items[0].listen == listen {
                    has = true;
                    n = i;
                    break;
                }
            }
            if has {
                configs[n].push(config);
            }else {
                configs.push(vec![config]);
            }

        }

        Ok(configs)

    }

}


