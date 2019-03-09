

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


fn main() {

    const ADDRESS: &str = "0.0.0.0:80";

    let listener = TcpListener::bind(ADDRESS);

    if let Err(listener) = listener {
        println!("{:?}", listener);
        println!("Binding {} failed.", ADDRESS);
        process::exit(1);
    }

    let server = listener.unwrap();
    for stream in server.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(|| {
                handle_connection(stream);
            });
        }
    }

}


fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]).to_string();
    let res = output(get_path(request));

    stream.write(&res).unwrap();
    stream.flush().unwrap();

}


fn get_path(request: String) -> String {

    let structure: Vec<&str> = request.split("\r\n").collect();
    let route: Vec<&str> = structure[0].split(" ").collect();

    route[1].replacen("/", "./", 1)

}


fn output(route: String) -> Vec<u8> {

    let meta = fs::metadata(&route);

    match meta {
        Ok(meta) => {
            if meta.is_dir() {
                let last = &route.chars().last().unwrap().to_string();
                if *last == String::from("/") {
                    return Response::new(StatusCode::Ok)
                        .content_type("html")
                        .body(response_dir_html(&route).as_bytes())
                }else {
                    let moved = route.replace(".", "") + "/";
                    return Response::new(StatusCode::Moved)
                        .header("location", &moved)
                        .body(String::from("").as_bytes())
                }
            }else {
                match fs::read(&route) {
                    Ok(data) => {
                        let ext = Path::new(&route).extension().unwrap().to_str().unwrap();
                        return Response::new(StatusCode::Ok)
                            .content_type(ext)
                            .body( &data[..])
                    },
                    Err(_) => {
                        return Response::new(StatusCode::Error)
                            .content_type("txt")
                            .body(String::from("500").as_bytes())
                    }
                }
            }
        },
        Err(_) => {
            return Response::new(StatusCode::NotFound)
                .content_type("txt")
                .body(String::from("404").as_bytes());
        }
    };

}


fn response_dir_html(path: &String) -> String {

    let dir = fs::read_dir(path).unwrap();
    let mut files = String::from("");
    for x in dir {
        let entry = x.unwrap().path();
        let filename = entry.file_name().unwrap().to_str().unwrap();
        files += &format!(r#"<li><a href="{}">{}</a></li>"#, filename, filename);
    }

    template()
        .replace("{title}", path)
        .replace("{list}", &files)

}

