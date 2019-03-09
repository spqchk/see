

use std::fs;
use std::thread;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::process;
use std::path::Path;
mod lib;
use lib::{Status, Response};


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

    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();

}


fn get_path(request: String) -> String {

    let vec: Vec<&str> = request.split("\n").collect();
    let line = vec[0].to_string();

    line
        .replace("GET /", "./")
        .replace(" HTTP/1.1\r", "")

}


fn output(route: String) -> String {

    let meta = fs::metadata(&route);

    match meta {
        Ok(meta) => {
            if meta.is_dir() {
                let last = &route.chars().last().unwrap().to_string();
                if *last == String::from("/") {
                    return Response::new(Status::Success)
                        .content_type("html")
                        .body(response_dir_html(&route))
                }else {
                    let moved = route.replace(".", "") + "/";
                    return Response::new(Status::Moved)
                        .header("location", &moved)
                        .body(String::from(""))
                }
            }else {
                let ext = Path::new(&route)
                    .extension()
                    .unwrap()
                    .to_str()
                    .unwrap();
                match fs::read(&route) {
                    Ok(data) => {
                        let text = String::from_utf8_lossy(&data).to_string();
                        return Response::new(Status::Success)
                            .content_type(ext)
                            .body(text)
                    },
                    Err(_) => {
                        return Response::new(Status::Error)
                            .content_type("txt")
                            .body(String::from("500"))
                    }
                }
            }
        },
        Err(_) => {
            return Response::new(Status::Not)
                .content_type("txt")
                .body(String::from("404"));
        }
    };

}


fn response_dir_html(path: &String) -> String {

    let template = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Dusk</title>
</head>
<body>
    <h1>Index of {title}</h1>
    <ul>
        <li><a href="../">../</a></li>
        {list}
    </ul>
</body>
</html>
    "#;

    let dir = fs::read_dir(path).unwrap();
    let mut files = String::from("");
    for x in dir {
        let entry = x.unwrap().path();
        let filename = entry.file_name().unwrap().to_str().unwrap();
        files += &format!(r#"<li><a href="{}">{}</a></li>"#, filename, filename);
    }

    template
        .replace("{title}", path)
        .replace("{list}", &files)
}

