

extern crate yaml_rust;
use yaml_rust::{YamlLoader};
use std::fs;
use std::result::Result;


#[derive(Debug)]
pub struct ServerConfig {
    default: bool,
    host: String,
    listen: i64,
    root: String,
    log: Log
}


#[derive(Debug)]
struct Log {
    success: String,
    error: String
}


impl ServerConfig {

    pub fn new(path: &str) -> Result<Vec<ServerConfig>, String>  {

        let str = fs::read_to_string(path).unwrap();

        let docs = YamlLoader::load_from_str(&str).unwrap();

        let mut configs: Vec<ServerConfig> = vec![];

        let servers = &docs[0].as_vec().unwrap();

//        let s = match &docs[0].as_vec() {
//            Some(d) => *d,
//            None => {
//
//            }
//        };

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

            configs.push(ServerConfig {
                default,
                host,
                listen,
                root,
                log: Log {
                    success,
                    error
                }
            });

        }

        Ok(configs)

    }

}


