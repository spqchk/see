

use std::u8;
use std::fs;
use std::thread;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::process;
use std::path::Path;
mod response;
use response::{StatusCode, Response};
mod html;
use html::template;
mod config;
use config::ServerConfig;
extern crate percent_encoding;
use percent_encoding::percent_decode;


fn main() {

    let configs = match ServerConfig::new("dusk.yml") {
        Ok(config) => config,
        Err(msg) => {
            println!("{}", msg);
            return;
        }
    };

    let mut wait = vec![];
    for config in configs {
        wait.push(thread::spawn(|| {
            bind_tcp(config);
        }));
    }

    for sp in wait {
        sp.join().unwrap();
    }

}


fn bind_tcp(config: Vec<ServerConfig>) {

    let listen = &config[0].listen;
    let address = format!("0.0.0.0:{}", listen);
    let listener = TcpListener::bind(&address);

    if let Err(listener) = listener {
        println!("{:?}", listener);
        println!("Binding {} failed.", &address);
        process::exit(1);
    }else {
        println!("Binding {} success.", &address);
    }

    let server = listener.unwrap();

    //    server.set_nonblocking(true)

    for stream in server.incoming() {
        if let Ok(stream) = stream {
//            thread::spawn(|| {
//                handle_connection(stream);
//            });
            handle_connection(stream, &config);
        }
    }

}


fn handle_connection(mut stream: TcpStream, config: &Vec<ServerConfig>) {

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    if u8::min_value() == buffer[0] {
        stream.flush().unwrap();
        return;
    }

    let request = String::from_utf8_lossy(&buffer[..]).to_string();
    let req = parse_connection(request);
    let mut res: Vec<u8> = vec![];

    for conf in config {
        // !!!
        let host = &req.host.replace(":1234", "");
        if &conf.host == host {
            res = output(&req.path, &conf);
            break;
        }
    }

    if res.len() == 0 {
        for conf in config {
            if conf.host == String::from("") {
                res = output(&req.path, &conf);
                break;
            }
        }
    }

    if res.len() != 0 {
        stream.write(&res).unwrap();
    }
    stream.flush().unwrap();

}


#[derive(Debug)]
struct Request {
    path: String,
    host: String
}

fn parse_connection(request: String) -> Request {

    let structure: Vec<&str> = request.split("\r\n").collect();
    let url: Vec<&str> = structure[0].split(" ").collect();
    let host: Vec<&str> = structure[1].split(" ").collect();

    let mut path = url[1].replacen("/", "./", 1);
    path = percent_decode(path.as_bytes())
        .decode_utf8()
        .unwrap()
        .to_string();

    Request {
        path,
        host: String::from(host[1])
    }

}


fn output(route: &String, config: &ServerConfig) -> Vec<u8> {

    let path_buff = Path::new(&config.root)
        .join(&route);

    let canonicalize = path_buff.canonicalize();
    let d = match canonicalize {
        Ok(p) => p,
        Err(_) => {
            return Response::new(StatusCode::NotFound)
                .content_type("txt")
                .body(b"404");
        }
    };

    let path = match d.to_str() {
        Some(s) => s,
        None => {
            return Response::new(StatusCode::NotFound)
                .content_type("txt")
                .body(b"404");
        }
    };

    match d.metadata() {
        Ok(meta) => {
            if meta.is_dir() {
                if get_last_string(&route) == String::from("/") {
                    return Response::new(StatusCode::Ok)
                        .content_type("html")
                        .body(response_dir_html(&path, &route).as_bytes())
                }else {
                    let moved = route.replacen(".", "", 1) + "/";
                    return Response::new(StatusCode::Moved)
                        .header("location", &moved)
                        .body(b"")
                }
            }else {
                match fs::read(&path) {
                    Ok(data) => {
                        return Response::new(StatusCode::Ok)
                            .content_type(get_ext(&path))
                            .body(&data[..])
                    },
                    Err(_) => {
                        return Response::new(StatusCode::Error)
                            .content_type("txt")
                            .body(b"500")
                    }
                }
            }
        },
        Err(_) => {
            return Response::new(StatusCode::NotFound)
                .content_type("txt")
                .body(b"404");
        }
    };

}


fn get_ext(route: &str) -> &str {

    let extension = Path::new(route)
        .extension();

    match extension {
        Some(ext) => {
            match ext.to_str() {
                Some(e) => e,
                None => ""
            }
        },
        None => ""
    }

}


fn get_last_string(route: &String) -> String {

    match route.chars().last() {
        Some(l) => l.to_string(),
        None => String::from("")
    }

}


fn response_dir_html(path: &str, title: &String) -> String {

    let dir = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(_) => {
            return String::from("")
        }
    };

    let mut files = String::from("");

    for x in dir {

        let entry = match x {
            Ok(entry) => entry,
            Err(_) => continue
        }.path();

        let filename = match entry.file_name() {
            Some(d) => {
                match d.to_str() {
                    Some(n) => n,
                    None => continue
                }
            },
            None => continue
        };

        files += &format!(r#"<li><a href="{}">{}</a></li>"#, filename, filename);

    }

    template()
        .replace("{title}", title)
        .replace("{list}", &files)

}

