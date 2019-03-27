

extern crate chrono;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::fs;
use chrono::{DateTime, prelude, Local};


#[derive(Debug)]
pub struct Log {
    pub file: File
}


fn create_log_file(path: String) -> File {
    let log_file = fs::metadata(&path);
    if let Err(_) = log_file {
        let p = Path::new(&path);
        let dir = &p.parent().unwrap();
        fs::create_dir_all(dir).unwrap();
        File::create(&path).unwrap();
    }
    OpenOptions::new()
        .write(true)
        .append(true)
        .open(&path)
        .unwrap()
}


impl Log {

    pub fn new(path: String) -> Log {
        Log {
            file: create_log_file(path)
        }
    }

    pub fn write(&self, method: &str, status: i32, path: &str) {
        let time: DateTime<Local> = prelude::Local::now();
        if let Err(e) = writeln!(&self.file, "{0}  {1: <6}  {2}  {3}", time, method, status, path) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

}


#[test]
fn test_log() {
    let log = Log::new("./logs/success.log".to_string());
    log.write("GET", 200, "/api");
    log.write("HEAD", 404, "/img");
    log.write("DELETE", 500, "/img");
}


