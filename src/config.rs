

extern crate yaml_rust;
use yaml_rust::{YamlLoader};
use std::fs;
use std::result::Result;
use crate::log::{Log as Da};
use crate::fill_path;

// Configuration of each service
#[derive(Debug, Default)]
pub struct ServerConfig {
    pub host: Option<String>,
    pub listen: i64,
    pub root: String,
    pub gzip: bool,
    pub directory: bool,
    pub index: Option<String>,
    pub headers: Vec<Header>,  // Option<Vec<Header>>
    pub extensions: Option<Vec<String>>,
    pub methods: Vec<String>,
    pub auth: Option<Auth>,
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
    pub not_found: Option<String>,
    pub error: Option<String>
}

// HTTP auth
#[derive(Debug, Default)]
pub struct Auth {
    pub user: String,
    pub password: String
}

// Log path
#[derive(Debug, Default)]
pub struct Log {
    pub success: Option<String>,
    pub error: Option<String>
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
                Some(d) => Some(d.to_string()),
                None => None
            };

            let listen = match &server["listen"].as_i64() {
                Some(d) => *d,
                None => {
                    return Err(String::from("Must bind port"));
                }
            };

            let root = match &server["root"].as_str() {
                Some(d) => d.to_string(),
                None => {
                    return Err(String::from("Must set root"));
                }
            };

            let gzip = match &server["gzip"].as_bool() {
                Some(d) => *d,
                None => false
            };

            let directory = match &server["directory"].as_bool() {
                Some(d) => *d,
                None => false
            };

            let index = match &server["index"].as_str() {
                Some(d) => Some(d.to_string()),
                None => None
            };

            let headers = match &server["headers"].as_hash() {
                Some(d) => {
                    let mut headers: Vec<Header> = vec![];
                    for (key, value) in d.iter() {
                        if let Some(k) = key.as_str() {
                            if let Some(v) = value.as_str() {
                                headers.push(Header {
                                    key: k.to_string(),
                                    value: v.to_string()
                                })
                            }else {
                                continue
                            }
                        }else {
                            continue
                        }
                    }
                    headers
                },
                None => vec![]
            };

            let extensions = match &server["extensions"].as_vec() {
                Some(extensions) => {
                    let mut vec: Vec<String> = vec![];
                    for item in extensions.iter() {
                        if let Some(ext) = item.as_str() {
                            vec.push(ext.to_string());
                        }
                    };
                    Some(vec)
                },
                None => None
            };

            let mut methods = vec![
                String::from("GET"),
                String::from("HEAD"),
            ];
            if let Some(d) = &server["methods"].as_vec() {
                for item in d.iter() {
                    if let Some(method) = item.as_str() {
                        methods.push(method.to_string());
                    }
                }
            }

            let error_not_found = match &server["error"][404].as_str() {
                Some(d) => Some(fill_path(&root, d)),
                None => None
            };

            let error_error = match &server["error"][500].as_str() {
                Some(d) => Some(fill_path(&root, d)),
                None => None
            };

            let log_success = match &server["log"]["success"].as_str() {
                Some(d) => Some(fill_path(&root, d)),
                None => None
            };

            let log_error = match &server["log"]["error"].as_str() {
                Some(d) => Some(fill_path(&root, d)),
                None => None
            };

            let user = match &server["auth"]["user"].as_str() {
                Some(d) => *d,
                None => ""
            }.to_string();

            let password = match &server["auth"]["password"].as_str() {
                Some(d) => *d,
                None => ""
            }.to_string();

            let auth = if user == "" || password == "" {
                None
            }else {
                Some(Auth {
                    user,
                    password
                })
            };

            let config = ServerConfig {
                host,
                listen,
                root,
                gzip,
                directory,
                index,
                headers,
                extensions,
                methods,
                error: Error {
                    not_found: error_not_found,
                    error: error_error
                },
                log: Log {
                    success: log_success,
                    error: log_error
                },
                auth
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


