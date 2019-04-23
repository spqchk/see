
# rock [![Build Status](https://img.shields.io/travis/wyhaya/rock.svg?style=flat-square)](https://travis-ci.org/wyhaya/rock)

rock is a static web server, developed using rust.

## Features

## Install

## Use

Quick Start
```bash
rock start
# or
rock start 8080
```

Start according to the configuration file
```bash
rock
# or
rock -c /your/config.yml
```

More
```
USAGE:
    rock [OPTIONS] [FLAGS] [--] ...

FLAGS:
    -d                  Running in the background
    -h, help            Print help information
    -s, stop            Stop the daemon
    -t                  Check the config file for errors
    -v, version         Print version number

OPTIONS:
    -c    <FILE>        Specify a configuration file
    start <PORT?>       Quick Start
```

## Config

Use `yaml` format as a configuration file, You can use `rock -c /your/config.yml` to specify the configuration file location.

Complete configuration file example: 

```yaml
- server:
    host: domain.com      # Domain name to be bound
    listen: 80            # Port to be monitored
    root: /root/www       # Directory that requires service
    index: index.html     # Index file
    directory:            # Whether to display the file list
      time: true
      size: true
    headers:              # Header in response
      Access-Control-Allow-Origin: *
      Set-Cookie: 12345
    gzip:                 # File type that needs to be compressed
      - html
      - css
    methods:              # Method of allowing requests
      - POST
      - PUT
    auth:                 # Http user and password verification
      user: name
      password: pwd
    extensions:           # Sets file extension fallbacks
      - html
      - htm
    error:                # Custom error page
      404: 404.html
      500: 500.html
    log:                  # Log save location
      error: /logs/domain.error.log
      success: /logs/domain.success.log
# More server ...
```

## Todo

* [ ] Async/response
* [ ] HTTP stream
* [ ] Gzip compress
* [ ] HTTPS / HTTP2
* [ ] Proxy
* [ ] Bind multiple domain names
* [ ] Bind multiple ports