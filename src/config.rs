

extern crate yaml_rust;
extern crate base64;

use std::fs;
use std::sync::Arc;
use std::result::Result;
use std::collections::HashMap;
use crate::log::Log;
use base64::encode;
use crate::fill_path;
use yaml_rust::{YamlLoader};

// Configuration of each service
#[derive(Debug, Default)]
pub struct ServerConfig {
    pub hosts: Option<Vec<String>>,
    pub listen: i64,
    pub root: String,
    pub gzip: Option<Vec<String>>,
    pub directory: Option<DirectoryOption>,
    pub index: Option<String>,
    pub headers: Vec<Header>,
    pub rewrite: Option<HashMap<String, Rewrite>>,
    pub extensions: Option<Vec<String>>,
    pub methods: Vec<String>,
    pub auth: Option<String>,
    pub error: Error,
    pub log: Recording
}

#[derive(Debug, Default)]
pub struct DirectoryOption {
    pub time: bool,
    pub size: bool
}

// Header -> key: value
#[derive(Debug, Default)]
pub struct Header {
    pub key: String,
    pub value: String
}

#[derive(Debug)]
pub struct Rewrite {
    pub url: String,
    pub status: RewriteType
}

#[derive(Debug)]
pub enum RewriteType {
    _301,
    _302,
    Path
}

// Error page
#[derive(Debug, Default)]
pub struct Error {
    pub _404: Option<String>,
    pub _500: Option<String>
}

// Log path
#[derive(Debug, Default)]
pub struct Recording {
    pub success: Option<Log>,
    pub error: Option<Log>
}


impl ServerConfig {

    // The same port service is a group
    pub fn new(path: &str) -> Result<Vec<Arc<Vec<ServerConfig>>>, String>  {

        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(err) => {
                return Err(format!("{} {}", path, err));
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

            let hosts = match server["host"].as_vec() {
                Some(vec) => {
                    let mut hosts = vec![];
                    for item in vec.iter() {
                        if let Some(d) = item.as_str() {
                            hosts.push(String::from(d));
                        }
                    }
                    Some(hosts)
                },
                None => {
                    if let Some(d) = server["host"].as_str() {
                        Some(vec![String::from(d)])
                    }else {
                        None
                    }
                }
            };

            let listen = match server["listen"].as_i64() {
                Some(d) => d,
                None => {
                    return Err(String::from("Must bind port"));
                }
            };

            let root = match server["root"].as_str() {
                Some(d) => d.to_string(),
                None => {
                    return Err(String::from("Must set root"));
                }
            };

            let gzip = match server["gzip"].as_vec() {
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

            let directory = match server["directory"].as_bool() {
                // No option
                Some(open) => {
                    if open {
                        Some(DirectoryOption::default())
                    }else {
                        None
                    }
                },
                None => {
                    match server["directory"].as_hash() {
                        // read option
                        Some(_) => {
                            let time = match server["directory"]["time"].as_bool() {
                                Some(time) => time,
                                None => false
                            };
                            let size = match server["directory"]["size"].as_bool() {
                                Some(size) => size,
                                None => false
                            };
                            Some(DirectoryOption {
                                time,
                                size
                            })
                        },
                        None => None
                    }
                }
            };

            let index = match server["index"].as_str() {
                Some(d) => Some(d.to_string()),
                None => None
            };

            let headers = match server["header"].as_hash() {
                Some(header) => {
                    let mut headers: Vec<Header> = vec![];
                    for (key, value) in header.iter() {
                        if let (Some(k), Some(v)) = (key.as_str(), value.as_str()){
                            headers.push(Header {
                                key: k.to_string(),
                                value: v.to_string()
                            })
                        }
                    }
                    headers
                },
                None => vec![]
            };

            let rewrite = match server["rewrite"].as_hash() {
                Some(rewrite) => {
                    let mut hash = HashMap::new();
                    for (key, value) in rewrite.iter() {
                        if let (Some(k), Some(v)) = (key.as_str(), value.as_str()){
                            let mut r = v.split_whitespace();
                            let url = match r.next() {
                                Some(url) => url.to_string(),
                                None => {
                                    return Err(String::from("Could not find redirected url"));
                                }
                            };
                            let status = match r.next() {
                                Some(aims) => match aims {
                                    "301" => RewriteType::_301,
                                    "302" => RewriteType::_302,
                                    "path" => RewriteType::Path,
                                    _ => {
                                        return Err(format!("Wrong redirect type \"{}\"", aims));
                                    }
                                },
                                None => RewriteType::_302
                            };
                            hash.insert(
                                k.to_string(),
                                Rewrite {
                                    url,
                                    status
                                }
                            );
                        }
                    }
                    Some(hash)
                },
                None => None
            };

            let extensions = match server["extension"].as_vec() {
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
            if let Some(vec) = server["method"].as_vec() {
                for item in vec.iter() {
                    if let Some(method) = item.as_str() {
                        methods.push(method.to_string());
                    }
                }
            }

            let _404 = match server["error"][404].as_str() {
                Some(d) => Some(fill_path(&root, d)),
                None => None
            };

            let _500 = match server["error"][500].as_str() {
                Some(d) => Some(fill_path(&root, d)),
                None => None
            };

            let success = match server["log"]["success"].as_str() {
                Some(d) => Some(Log::new(fill_path(&root, d))),
                None => None
            };

            let error = match server["log"]["error"].as_str() {
                Some(d) => Some(Log::new(fill_path(&root, d))),
                None => None
            };

            let auth = match server["auth"].as_hash() {
                Some(_) => {
                    let user = match server["auth"]["user"].as_str() {
                        Some(d) => d,
                        None => {
                            return Err(String::from("Missing 'user' in auth"));
                        }
                    };
                    let password = match server["auth"]["password"].as_str() {
                        Some(d) => d,
                        None => {
                            return Err(String::from("Missing 'password' in auth"));
                        }
                    };
                    let s = format!("{}:{}", user, password);
                    let base64 = format!("Basic {}", encode(&s));
                    Some(base64)
                },
                None => None
            };

            let config = ServerConfig {
                hosts,
                listen,
                root,
                gzip,
                directory,
                index,
                headers,
                rewrite,
                extensions,
                methods,
                error: Error {
                    _404,
                    _500
                },
                log: Recording {
                    success,
                    error
                },
                auth
            };

            let (mut has, mut n) = (false, 0);
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

        let mut conf = vec![];
        for config in configs {
            conf.push(Arc::new(config));
        }

        Ok(conf)

    }

}


