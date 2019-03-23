#![feature(futures_api)]
#![feature(async_await)]

extern crate base64;
use base64::{encode, decode};
use std::u8;
use std::fs;
use std::env;
use std::thread;
use std::process;
use std::path::Path;
use std::io::prelude::*;
use futures::executor::block_on;
use std::net::{TcpStream, TcpListener};
mod response;
use response::{StatusCode, Response};
mod request;
use request::Request;
mod html;
use html::TEMPLATE;
mod config;
use config::ServerConfig;
mod log;


fn main() {

    let mut config_path = match get_arg(String::from("-c")) {
        Some(p) => p,
        None => String::from("./sws.yml")
    };

    let config_buf = env::current_dir()
        .unwrap();
    let root = config_buf
        .to_str()
        .unwrap();

    config_path = fill_path(root, &config_path);

    let configs = match ServerConfig::new(config_path) {
        Ok(config) => config,
        Err(msg) => {
            eprintln!("\x1b[31msws: {}\x1b[0m", msg);
            process::exit(1);
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


fn get_arg(name: String) -> Option<String> {

    let args: Vec<String> = env::args().collect();
    let c = &args[1..];
    let mut arg = "";

    for (i, x) in c.iter().enumerate() {
        if x == &name && c.len() - 1 > i {
            arg = c[i + 1].as_str();
            break;
        }
    }

    match arg {
        "" => None,
        _ => Some(arg.to_string())
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
            let future = handle_connection(stream, &config);
            block_on(future);
        }
    }

}


async fn handle_connection(mut stream: TcpStream, config: &Vec<ServerConfig>) {

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    if u8::min_value() == buffer[0] {
        stream.flush().unwrap();
        return;
    }

    let mut res: Vec<u8> = vec![];
    let req = Request::new(&buffer[..]);

    match req.headers.get("Host") {
        Some(host) => {
            for conf in config {
                let rm_listen = format!(":{}", &conf.listen);
                if &conf.host == &host.replace(rm_listen.as_str(), "") {
                    res = output(&req, &conf);
                    break;
                }
            }
        },
        None => { }
    };

    if res.len() == 0 {
        for conf in config {
            if conf.host == String::from("") {
                res = output(&req, &conf);
                break;
            }
        }
    }

    if res.len() != 0 {
        stream.write(&res).unwrap();
    }
    stream.flush().unwrap();

}


fn output(request: &Request, config: &ServerConfig) -> Vec<u8> {

    //    Do you need authentication
    if let Some(auth) = &config.auth {
        let authorization = request.headers.get("Authorization");
        if let Some(value) = authorization {
            let config_auth = format!("{}:{}", auth.user, auth.password);
            // Support multiple ways ?
            let base64 = value.replacen("Basic ", "", 1);
            let base64_bytes = decode(&base64).unwrap();
            if config_auth.as_bytes() != base64_bytes.as_slice() {
                return Response::new(StatusCode::_401)
                    .content_type("")
                    .header("WWW-Authenticate", "Basic realm=\"User Visible Realm\"")
                    .body(b"401");
            }
        }else {
            return Response::new(StatusCode::_401)
                .content_type("")
                .header("WWW-Authenticate", "Basic realm=\"Need to verify identity\"")
                .body(b"401");
        }
    }

    let cur_path = String::from(".") + &request.path;
    let path_buff = Path::new(&config.root)
        .join(&cur_path);
    let path = path_buff
        .to_str()
        .unwrap();

    // A Host header field must be sent in all HTTP/1.1 request messages
    match request.headers.get("Host") {
        None => {
            return Response::new(StatusCode::_400)
                .content_type("txt")
                .body(b"400");
        },
        _ => {}
    }

    match fs::metadata(&path) {
        Ok(meta) => {
            if meta.is_dir() {
                if get_last_string(&request.path) == String::from("/") {
                    if &config.index != "" {
                        let index_path = fill_path(&path, &config.index);
                        match fs::read(index_path) {
                            Ok(data) => {
                                return Response::new(StatusCode::_200)
                                    .content_type(get_extension(&config.index))
                                    .headers(&config.headers)
                                    .body(&data[..])
                            },
                            Err(_) => {
                                return output_not_found(&config);
                            }
                        }
                    }
                    if config.directory {
                        return Response::new(StatusCode::_200)
                            .content_type("html")
                            .headers(&config.headers)
                            .body(response_dir_html(&path, &request.path).as_bytes())
                    }
                    return output_not_found(&config);
                }else {
                    let moved = format!("{}/", request.path);
                    return Response::new(StatusCode::_301)
                        .header("location", &moved)
                        .headers(&config.headers)
                        .body(b"")
                }
            }else {
                match fs::read(&path) {
                    Ok(data) => {
                        return Response::new(StatusCode::_200)
                            .content_type(get_extension(&path))
                            .headers(&config.headers)
                            .body(&data[..])
                    },
                    Err(_) => {
                        return output_error(&config);
                    }
                }
            }
        },
        Err(_) => {
            match fallbacks(&path, &config.extensions) {
                Ok(fallback) => {
                    return Response::new(StatusCode::_200)
                        .content_type(get_extension(&fallback.1))
                        .headers(&config.headers)
                        .body(&fallback.0[..]);
                },
                Err(_) => {
                    return output_not_found(&config);
                }
            }
        }
    };

}


pub fn fill_path(root: &str, file: &str) -> String {

    if Path::new(&file).is_absolute() {
        file.to_string()
    } else {
        let buff = Path::new(&root)
            .join(&file);
        let path = buff
            .to_str()
            .unwrap();
        path.to_string()
    }

}


fn output_not_found(config: &ServerConfig) -> Vec<u8> {

    let res = Response::new(StatusCode::_404)
        .headers(&config.headers);

    if &config.error.not_found == "" {
        return res
                .content_type("txt")
                .body(b"404")
    }

    match fs::read(&config.error.not_found) {
            Ok(data) => {
                return res
                    .content_type(get_extension(&config.error.not_found))
                    .body(&data[..])
            },
            Err(_) => {
                return res
                    .content_type("txt")
                    .body(b"404");
            }
        }

}


fn output_error(config: &ServerConfig) -> Vec<u8> {

    let res = Response::new(StatusCode::_500)
        .headers(&config.headers);

    if &config.error.error == "" {
        return res
            .content_type("txt")
            .body(b"500")
    }

    match fs::read(&config.error.error) {
        Ok(data) => {
            return res
                .content_type(get_extension(&config.error.error))
                .body(&data[..])
        },
        Err(_) => {
            return res
                .content_type("txt")
                .body(b"500");
        }
    }

}

// Get the file extension
fn get_extension(path: &str) -> &str {

    let extension = Path::new(path)
        .extension();
    
    if let Some(ext) = extension {
        match ext.to_str() {
            Some(e) => e,
            None => ""
        }
    } else {
        ""
    }

}

#[test]
fn test_get_extension() {
    assert_eq!(get_extension("index.html"), "html");
    assert_eq!(get_extension("/index/index.rs"), "rs");
    assert_eq!(get_extension(""), "");
    assert_eq!(get_extension("index"), "");
}


// Get the last character
fn get_last_string(path: &str) -> String {

    if let Some(last) = path.chars().last() {
        last.to_string()
    }else {
        String::from("")
    }

}


struct Fallbacks (
    Vec<u8>,
    String
);

fn fallbacks(file: &str, exts: &Vec<String>) -> Result<Fallbacks, ()> {

    let has_ext = Path::new(&file)
        .extension();
    match has_ext {
        Some(_) => {
            return Err(());
        },
        None => { }
    }

    for x in exts {
        let path = format!("{}.{}", file, x);
        match fs::read(&path) {
            Ok(data) => {
                return Ok(Fallbacks(data, path));
            },
            Err(_) => {}
        }
    }

    return Err(());

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

    TEMPLATE
        .replace("{title}", title)
        .replace("{files}", &files)

}


