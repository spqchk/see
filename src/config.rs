

extern crate yaml_rust;
use yaml_rust::{YamlLoader};
use std::fs;
use std::result::Result;


#[derive(Debug, Default)]
pub struct ServerConfig {
    pub host: String,
    pub listen: i64,
    pub root: String,
    pub gzip: bool,
    pub directory: bool,
    pub index: String,
    pub headers: Vec<Vec<String>>,
    pub extensions: Vec<String>,
    pub error: Error,
    pub log: Log
}


#[derive(Debug, Default)]
pub struct Error {
    pub not_found: String,
    pub error: String
}


#[derive(Debug, Default)]
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

            let gzip = match &server["gzip"].as_bool() {
                Some(d) => *d,
                None => false
            };

            let directory = match &server["directory"].as_bool() {
                Some(d) => *d,
                None => false
            };

            let index = match &server["index"].as_str() {
                Some(d) => *d,
                None => ""
            }.to_string();

            let headers = match &server["headers"].as_vec() {
                Some(header) => {
                    let mut vec: Vec<Vec<String>> = vec![];
                    for x in header.iter() {
                        let a = x.as_str().unwrap();
                        let v: Vec<&str> = a.split(" ").collect();
                        vec.push(vec![v[0].to_string(), v[1].to_string()]);
                    }
                    vec
                },
                None => vec![]
            };

            let extensions = match &server["extensions"].as_vec() {
                Some(exts) => {
                    let mut vec: Vec<String> = vec![];
                    for x in exts.iter() {
                        let e = x.as_str().unwrap();
                        vec.push(e.to_string());
                    }
                    vec
                },
                None => vec![]
            };

            let error_not_found = match &server["error"][404].as_str() {
                Some(d) => *d,
                None => ""
            }.to_string();

            let error_error = match &server["error"][500].as_str() {
                Some(d) => *d,
                None => ""
            }.to_string();

            let log_success = match &server["log"]["success"].as_str() {
                Some(d) => *d,
                None => ""
            }.to_string();

            let log_error = match &server["log"]["error"].as_str() {
                Some(d) => *d,
                None => ""
            }.to_string();

            let config = ServerConfig {
                host,
                listen,
                root,
                gzip,
                directory,
                index,
                headers,
                extensions,
                error: Error {
                    not_found: error_not_found,
                    error: error_error
                },
                log: Log {
                    success: log_success,
                    error: log_error
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


