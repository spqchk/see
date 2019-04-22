

extern crate chrono;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::fs;
use chrono::{DateTime, prelude, Local};


#[derive(Debug)]
pub struct Log {
    file: Option<File>
}


fn create_log_file(path: String) -> Option<File> {

    if let Err(_) = File::open(&path) {
        let parent = match Path::new(&path).parent() {
            Some(p) => p,
            None => {
                return None;
            }
        };
        if let Err(_) = fs::create_dir_all(parent) {
            return None;
        }
        if let Err(_) = File::create(&path) {
            return None;
        }
    }

    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&path);

    match file {
        Ok(file) => Some(file),
        Err(_) => None
    }

}


impl Log {

    pub fn new(path: String) -> Log {
        Log {
            file: create_log_file(path)
        }
    }

    pub fn write(&self, method: &str, status: i32, path: &str)  {

        let mut file = if let Some(file) = &self.file {
            file.try_clone().unwrap()
        }else {
            return;
        };
        let method = String::from(method);
        let path = String::from(path);

        runtime::spawn(async move {
            let time: DateTime<Local> = prelude::Local::now();
            if let Err(e) = writeln!(file, "{0}  {1: <6}  {2}  {3}", time, method, status, path) {
                eprintln!("Couldn't write to file: {}", e);
            }
        });

    }

}


#[test]
fn test_log() {
    let log = Log::new(String::from("./logs/test.log"));
    log.write("GET", 200, "/api");
    log.write("HEAD", 404, "/img");
    log.write("DELETE", 500, "/img");
}


