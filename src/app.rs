

use std::env;

pub struct App {
    pub args: Vec<String>
}

impl App {

    pub fn new() -> App {
        App {
            args: env::args().collect()
        }
    }

    pub fn detach(&self) -> bool {
        self.get_arg_flag("-d") || self.get_arg_flag("detach")
    }

    pub fn detach_args(&self) -> [&str; 2] {
        ["-d", "detach"]
    }

    pub fn help(&self) -> bool {
        self.get_arg_flag("-h") || self.get_arg_flag("help")
    }

    pub fn stop(&self) -> bool {
        self.get_arg_flag("-s") || self.get_arg_flag("stop")
    }

    pub fn test(&self) -> bool {
        self.get_arg_flag("-t") || self.get_arg_flag("test")
    }

    pub fn version(&self) -> bool {
        self.get_arg_flag("-v") || self.get_arg_flag("version")
    }

    pub fn config(&self) -> Option<String> {
        self.get_arg_option("-c")
    }

    pub fn start(&self) -> bool {
        self.get_arg_flag("start")
    }

    pub fn port(&self) -> Result<Option<i64>, String> {
        if let Some(port) = self.get_arg_option("start") {
            if let Ok(port) = port.parse::<i64>() {
                Ok(Some(port))
            }else {
                Err(port)
            }
        }else {
            Ok(None)
        }
    }

    pub fn print_help(&self) {
        print!(r#"{0} version {1}
{2}

USAGE:
    {0} [OPTIONS] [FLAGS] [--] ...

FLAGS:
    -d, detach          Running in the background
    -h, help            Print help information
    -s, stop            Stop the daemon
    -t, test            Test the config file for error
    -v, version         Print version number

OPTIONS:
    -c    <FILE>        Specify a configuration file
    start <PORT?>       Quick Start
"#,
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS")
        );
    }

    pub fn print_version(&self) {
        println!("{} version {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    }

    fn get_arg_flag(&self, flag: &str) -> bool {
        for x in self.args.iter() {
            if x == &flag {
                return true;
            }
        }
        false
    }

    fn get_arg_option(&self, arg: &str) -> Option<String> {
        for (i, x) in self.args.iter().enumerate() {
            if arg == x && self.args.len() > i + 1 {
                return Some(self.args[i + 1].to_string());
            }
        }
        None
    }

}


