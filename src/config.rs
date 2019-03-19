

extern crate yaml_rust;
use yaml_rust::{YamlLoader};
use std::fs;
use std::result::Result;

// Configuration of each service
#[derive(Debug, Default)]
pub struct ServerConfig {
    pub host: String,
    pub listen: i64,
    pub root: String,
    pub gzip: bool,
    pub directory: bool,
    pub index: String,
    pub headers: Vec<Header>,
    pub extensions: Vec<String>,
    pub error: Error,
    pub log: Log
}

// Header -> key: value
#[derive(Debug, Default)]
pub struct Header {
    pub key: String,
    pub value: String
}

// Error page
#[derive(Debug, Default)]
pub struct Error {
    pub not_found: String,
    pub error: String
}

// Log path
#[derive(Debug, Default)]
pub struct Log {
    pub success: String,
    pub error: String
}


impl ServerConfig {

    // The same port service is a group
    pub fn new(path: String) -> Result<Vec<Vec<ServerConfig>>, String>  {

        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(err) => {
                return Err(err.to_string());
            }
        };

        let docs = match YamlLoader::load_from_str(&content) {
            Ok(yaml) => {
                if yaml.len() == 0 {
                    return Err(String::from("Server should be a list"));
                }
                yaml
            },
            Err(err) => {
                return Err(err.to_string());
            }
        };

        let mut configs: Vec<Vec<ServerConfig>> = vec![];

        let vec = &docs[0].as_vec();
        let servers = match vec {
            Some(servers) => servers,
            None => {
                return Err(String::from("Server should be a list"));
            }
        };

        for x in servers.iter() {

            let server = &x["server"];

            let host = match &server["host"].as_str() {
                Some(d) => *d,
                None => ""
            }.to_string();

            let listen = match &server["listen"].as_i64() {
                Some(d) => *d,
                None => {
                    return Err(String::from("Must bind port"));
                }
            };

            let root = match &server["root"].as_str() {
                Some(d) => *d,
                None => {
                    return Err(String::from("Must set root"));
                }
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
                    let mut vec: Vec<Header> = vec![];
                    for item in header.iter() {
                        let header = match item.as_str() {
                            Some(d) => d,
                            None => {
                                return Err(String::from("Header should be a string"));
                            }
                        };
                        let split: Vec<&str> = header.split(" ").collect();
                        if split.len() != 2 {
                            return Err(String::from("Key and value in the header are separated by a space"));
                        }
                        vec.push(Header {
                            key: split[0].to_string(),
                            value: split[1].to_string()
                        });
                    }
                    vec
                },
                None => vec![]
            };

            let extensions = match &server["extensions"].as_vec() {
                Some(extensions) => {
                    let mut vec: Vec<String> = vec![];
                    for item in extensions.iter() {
                        let ext = match item.as_str() {
                            Some(d) => d,
                            None => {
                                return Err(String::from("Each item in extensions should be a string"));
                            }
                        };
                        vec.push(ext.to_string());
                    };
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


