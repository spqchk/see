

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::fs;
use yaml_rust::yaml::Yaml::Null;
use futures::executor::block_on;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Log {
    success: Option<File>,
    error: Option<File>
}


fn create_log_file(path: &str) -> File {
    let log_file = fs::metadata(&path);
    if let Err(log_file) = log_file {
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

    fn new(success: &str, error: &str) -> Log {
        Log {
            success: if success == "" {
                None
            }else {
                Some(create_log_file(success))
            },
            error: if error == "" {
                None
            }else {
                Some(create_log_file(error))
            }
        }
    }

    fn success(self, text: &str) {
        let sync = self.write(text);
        block_on(sync);
    }

    async fn write(self, text: &str) {
        if let Some(file) = self.success {
//            thread::sleep(Duration::from_secs(5));
            if let Err(e) = writeln!(&file, "{}", &text) {
                eprintln!("Couldn't write to file: {}", e);
            }else {
                println!("写入完成");
            }
        }
    }

}


#[test]
fn test_log() {
//    let log = Log::new("./logs/success.log", "./logs/error.log");
//    log.success("hello world -- a");
//    assert_eq!(true, false);
}


