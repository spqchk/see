

#![feature(async_await, await_macro)]

extern crate chrono;
use std::u8;
use std::{fs, fs::File};
use std::env;
use std::{process, process::Command};
use std::path::Path;
use std::io::prelude::*;
use std::fmt::Write as FmtWrite;
use runtime::task::JoinHandle;
use std::net::{TcpStream, TcpListener};
use chrono::{DateTime, Local};
mod response;
use response::{StatusCode, Response};
mod request;
use request::Request;
mod html;
use html::TEMPLATE;
mod config;
use config::{ServerConfig, DirectoryOption};
mod log;


#[cfg(target_os = "macos")]
static PID_PATH: &str = "/usr/local/var/run/see.pid";
#[cfg(target_os = "linux")]
static PID_PATH: &str = "/var/run/see.pid";
#[cfg(target_os = "windows")]
static PID_PATH: &str = "./see.pid";

const DEFAULT_CONFIG_PATH: &str = "config.yml";

#[runtime::main]
async fn main() {

    //  Print help information
    if get_arg_flag("-h") || get_arg_flag("help") {
        return print!(r#"{0} version {1}
{2}

USAGE:
    {0} [OPTIONS] [FLAGS] [--] ...

FLAGS:
    -d                  Running in the background
    -h, help            Print help information
    -s, stop            Stop the daemon
    -t                  Check the config file for errors
    -v, version         Print version number

OPTIONS:
    -c    <FILE>        Specify a configuration file
    start <PORT>?       Quick Start
"#,
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS")
        );
    }

    // Print version number
    if get_arg_flag("-v") || get_arg_flag("version") {
        return println!("{} version {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    }

    // Stop the daemon
    if get_arg_flag("-s") || get_arg_flag("stop") {
        return stop_daemon();
    }

    let mut configs: Vec<Vec<ServerConfig>>;
    let current_buff = env::current_dir()
        .unwrap();
    let current_dir = current_buff.to_str()
        .unwrap();

    if get_arg_flag("start") {

        let mut config = ServerConfig::default();

        config.root = String::from(current_dir);
        config.directory = Some(DirectoryOption {
            time: true,
            size: true
        });
        config.methods = vec![
            String::from("GET"),
            String::from("HEAD"),
        ];
        config.listen = match get_arg_option("start") {
            Some(port) => {
                if let Ok(p) = port.parse::<i64>() {
                    p
                }else {
                    eprintln!("unable to bind to port {}", port);
                    process::exit(1);
                }
            },
            None => 80
        };

        configs = vec![vec![config]];

    }else {

        let mut config_path = match get_arg_option("-c") {
            Some(p) => p,
            None => String::from(DEFAULT_CONFIG_PATH)
        };

        config_path = fill_path(current_dir, &config_path);
        configs = match ServerConfig::new(&config_path) {
            Ok(config) => config,
            Err(msg) => {
                eprintln!("{}", msg);
                process::exit(1);
            }
        };

        // Check configuration file
        if get_arg_flag("-t") {
            return println!("the configuration file {} syntax is ok", config_path);
        }

    }

    // Running in the background
    if get_arg_flag("-d") {
        return start_daemon();
    }

    let mut tasks: Vec<JoinHandle<()>> = vec![];
    for config in configs {
        let run = runtime::spawn(async move {
            bind_tcp(config);
        });
        tasks.push(run);
    }
    for task in tasks {
        await!(task);
    }

}


fn get_arg_option(arg: &str) -> Option<String> {
    let args: Vec<String> = env::args().collect();
    for (i, x) in args.iter().enumerate() {
        if arg == x && args.len() > i + 1 {
            return Some(args[i + 1].to_string());
        }
    }
    None
}


fn get_arg_flag(flag: &str) -> bool {
    let args: Vec<String> = env::args().collect();
    for x in args.iter() {
        if x == &flag {
            return true;
        }
    }
    false
}


fn start_daemon() {
    let args: Vec<String> = env::args().collect();
    let args: Vec<String> = args.iter().filter(|item| *item != "-d").cloned().collect();
    let child = Command::new(&args[0])
        .args(&args[1..])
        .spawn();
    match child {
        Ok(child) => {
            let mut pid = File::create(PID_PATH)
                .unwrap();
            write!(pid, "{}", child.id()).unwrap();
        },
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}


fn stop_daemon() {
    match fs::read_to_string(PID_PATH) {
        Ok(pid) => {
            let kill = Command::new("kill")
                .arg(pid)
                .status();
            match kill {
                Ok(status) => {
                    if status.success() {
                        if let Err(e) = fs::remove_file(PID_PATH) {
                            eprintln!("{}", e);
                        }
                    }
                },
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        },
        Err(e) => {
            eprintln!("open \"{}\" failed, {:?}", PID_PATH, e.to_string());
        }
    }
}


fn bind_tcp(config: Vec<ServerConfig>) {

    let listen = config[0].listen;
    let address = format!("0.0.0.0:{}", listen);
    let listener = TcpListener::bind(&address);

    if let Err(listener) = listener {
        eprintln!("{:?}", listener);
        eprintln!("Binding {} failed", address);
        process::exit(1);
    }

    for stream in listener.unwrap().incoming() {
        if let Ok(stream) = stream {
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

    let mut res: Vec<u8> = vec![];
    let req = Request::new(&buffer[..]);

    if let Some(host) = req.headers.get("host") {
        for conf in config {
            let rm_listen = format!(":{}", &conf.listen);
            if let Some(val) = &conf.host {
                if val == &host.replace(rm_listen.as_str(), "") {
                    res = output(&req, &conf);
                    break;
                }
            }
        }
    }

    if res.len() == 0 {
        for conf in config {
            if let None = conf.host {
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

    // Not allowed method
    let allow = config.methods.iter().find(|item| {
        return **item == request.method;
    });
    if let None = allow {
        if let Some(log) = &config.log.error {
            log.write(&request.method, 405, &request.path);
        }
        return Response::new(StatusCode::_405)
            .content_type("txt")
            .headers(&config.headers)
            .body(b"405");
    }

    // A Host header field must be sent in all HTTP/1.1 request messages
    if let None = request.headers.get("host") {
        if let Some(log) = &config.log.error {
            log.write(&request.method, 400, &request.path);
        }
        return Response::new(StatusCode::_400)
            .content_type("txt")
            .headers(&config.headers)
            .body(b"400");
    }

    // Do you need authentication
    if let Some(auth) = &config.auth {
        let authorization = request.headers.get("authorization");
        if let Some(value) = authorization {
            // Support multiple ways ?
            if auth != value {
                if let Some(log) = &config.log.error {
                    log.write(&request.method, 401, &request.path);
                }
                return Response::new(StatusCode::_401)
                    .content_type("txt")
                    .header("WWW-Authenticate", "Basic realm=\"User Visible Realm\"")
                    .headers(&config.headers)
                    .body(b"401");
            }
        }else {
            if let Some(log) = &config.log.error {
                log.write(&request.method, 401, &request.path);
            }
            return Response::new(StatusCode::_401)
                .content_type("txt")
                .header("WWW-Authenticate", "Basic realm=\"User Visible Realm\"")
                .headers(&config.headers)
                .body(b"401");
        }
    }

    let cur_path = String::from(".") + &request.path;
    let path_buff = Path::new(&config.root)
        .join(&cur_path);
    let path = path_buff
        .to_str()
        .unwrap();

    match fs::metadata(&path) {
        Ok(meta) => {
            if meta.is_dir() {
                if get_last_string(&request.path) == String::from("/") {
                    if let Some(index) = &config.index {
                        let index_path = fill_path(&path, &index);
                        match fs::read(index_path) {
                            Ok(data) => {
                                if let Some(log) = &config.log.success {
                                    log.write(&request.method, 200, &request.path);
                                }
                                let ext = get_extension(index);
                                return Response::new(StatusCode::_200)
                                    .content_type(ext)
                                    .headers(&config.headers)
                                    .gzip(can_use_gzip(&request, &config, &ext))
                                    .body(&data[..])
                            },
                            Err(_) => {
                                if let Some(log) = &config.log.error {
                                    log.write(&request.method, 404, &request.path);
                                }
                                return output_404(&config);
                            }
                        }
                    }
                    if let Some(option) = &config.directory {
                        if let Some(log) = &config.log.success {
                            log.write(&request.method, 200, &request.path);
                        }
                        return Response::new(StatusCode::_200)
                            .content_type("html")
                            .headers(&config.headers)
                            .body(response_dir_html(&path, &request.path, option.time, option.size).as_bytes())
                    }
                    if let Some(log) = &config.log.error {
                        log.write(&request.method, 404, &request.path);
                    }
                    return output_404(&config);
                }else {
                    if let Some(log) = &config.log.success {
                        log.write(&request.method, 301, &request.path);
                    }
                    let moved = format!("{}/", request.path);
                    return Response::new(StatusCode::_301)
                        .header("location", &moved)
                        .headers(&config.headers)
                        .body(b"")
                }
            }else {
                match fs::read(&path) {
                    Ok(data) => {
                        if let Some(log) = &config.log.success {
                            log.write(&request.method, 200, &request.path);
                        }
                        let ext = get_extension(&path);
                        return Response::new(StatusCode::_200)
                            .content_type(&ext)
                            .headers(&config.headers)
                            .gzip(can_use_gzip(&request, &config, &ext))
                            .body(&data[..])
                    },
                    Err(_) => {
                        if let Some(log) = &config.log.error {
                            log.write(&request.method, 500, &request.path);
                        }
                        return output_500(&config);
                    }
                }
            }
        },
        Err(_) => {
            if let Some(exts) = &config.extensions {
                match fallbacks(&path, exts) {
                    Ok(fallback) => {
                        if let Some(log) = &config.log.success {
                            log.write(&request.method, 200, &request.path);
                        }
                        return Response::new(StatusCode::_200)
                            .content_type(&fallback.ext)
                            .headers(&config.headers)
                            .gzip(can_use_gzip(&request, &config, &fallback.ext))
                            .body(&fallback.data);
                    },
                    Err(_) => {
                        if let Some(log) = &config.log.error {
                            log.write(&request.method, 404, &request.path);
                        }
                        return output_404(&config);
                    }
                }
            }else {
                if let Some(log) = &config.log.error {
                    log.write(&request.method, 404, &request.path);
                }
                return output_404(&config);
            }
        }
    };

}


fn can_use_gzip(request: &Request, config: &ServerConfig, ext: &str) -> bool {

    if let Some(exts) = &config.gzip {
        let allow = exts.iter().find(|item| {
            return *item == ext
        });
        if let None = allow {
            return false
        }
    }else {
        return false
    }

    let encoding = if let Some(val) = request.headers.get("accept-encoding") {
        val
    }else {
        return false
    };

    let ways: Vec<&str> = encoding.split(", ").collect();
    for way in ways {
        if way == "gzip" {
            return true
        }
    }

    return false

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


fn output_404(config: &ServerConfig) -> Vec<u8> {

    let res = Response::new(StatusCode::_404)
        .headers(&config.headers);

    if let Some(file) = &config.error._404 {
        match fs::read(&file) {
            Ok(data) => {
                return res
                    .content_type(get_extension(file))
                    .body(&data[..])
            },
            Err(_) => {
                return res
                    .content_type("txt")
                    .body(b"404");
            }
        }
    }else {
        return res
            .content_type("txt")
            .body(b"404")
    }

}


fn output_500(config: &ServerConfig) -> Vec<u8> {

    let res = Response::new(StatusCode::_500)
        .headers(&config.headers);

    if let Some(file) = &config.error._500 {
        match fs::read(&file) {
            Ok(data) => {
                return res
                    .content_type(get_extension(file))
                    .body(&data[..])
            },
            Err(_) => {
                return res
                    .content_type("txt")
                    .body(b"500");
            }
        }
    }else {
        return res
            .content_type("txt")
            .body(b"500")
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


struct Fallbacks {
    data: Vec<u8>,
    ext: String
}

fn fallbacks(file: &str, exts: &Vec<String>) -> Result<Fallbacks, ()> {

    let has_ext = Path::new(&file)
        .extension();
    if let Some(_) = has_ext {
        return Err(());
    }

    for x in exts {
        let path = format!("{}.{}", file, x);
        if let Ok(data) = fs::read(&path) {
            return Ok(Fallbacks {
                data,
                ext: x.to_string()
            });
        }
    }

    return Err(());

}


fn response_dir_html(path: &str, title: &str, show_time: bool, show_size: bool) -> String {

    let dir = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(_) => {
            return String::new()
        }
    };

    let (
        mut files,
        mut main,
        mut first
    ) = (
        String::new(),
        "auto auto 1fr",
        "1 / 4"
    );

    if !show_time && !show_size{
        main = "auto";
        first = "1 / 2";
    }else if (!show_time && show_size) || (show_time && !show_size) {
        main = "auto 1fr";
        first = "1 / 3";
    }

    for x in dir {

        let entry = match x {
            Ok(entry) => entry,
            Err(_) => continue
        }.path();

        let filename = match entry.file_name() {
            Some(d) => {
                match d.to_str() {
                    Some(n) => {
                        if entry.is_dir() {
                            format!("{}/", n)
                        }else {
                            n.to_string()
                        }
                    },
                    None => continue
                }
            },
            None => continue
        };

        let _ = write!(files, "<a href=\"{}\">{}</a>", filename, filename);

        if show_size || show_time {
            if let Ok(meta) = fs::metadata(&entry) {
                if show_time {
                    let time = if let Ok(time) = meta.modified() {
                        let datetime: DateTime<Local> = DateTime::from(time);
                        datetime.format("%Y-%m-%d %H:%M").to_string()
                    }else {
                        String::new()
                    };
                    files.push_str(&format!("<time>{}</time>", time));
                }
                if show_size {
                    let size = if entry.is_file() {
                        bytes_to_size(meta.len() as f64)
                    }else {
                        String::new()
                    };
                    files.push_str(&format!("<span>{}</span>", size));
                }
            }
        }

    }

    TEMPLATE
        .replace("{title}", title)
        .replace("{main}", main)
        .replace("{first}", first)
        .replace("{files}", &files)

}


fn bytes_to_size(bytes: f64) -> String {
    let k = 1024_f64;
    let sizes = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let i = (bytes.ln() / k.ln()) as i32;
    format!("{:.2} {}", bytes / k.powi(i), sizes[i as usize])
}


