
# see [![Build Status](https://img.shields.io/travis/wyhaya/see.svg?style=flat-square)](https://travis-ci.org/wyhaya/see)

see is a static web server, developed using rust.

## Features

## Install

## Use

Quick Start
```bash
see start
# or
see start 8080
```

Start according to the configuration file
```bash
see
# or
see -c /your/config.yml
```

More
```
USAGE:
    see [OPTIONS] [FLAGS] [--] ...

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

Use `yaml` format as a configuration file, You can use `see -c /your/config.yml` to specify the configuration file location.

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
    header:               # Header in response
      Access-Control-Allow-Origin: "*"
      Set-Cookie: "12345"
    rewrite:              # Default 302 
      /img: /images 301
      /url: https://example.com 302
      /html: /index.html path
    compress:             # File type that needs to be compressed
      mode: gzip
      extension:
        - css
        - js
    method:               # Method of allowing requests
      - POST
      - PUT
    auth:                 # Http user and password verification
      user: name
      password: pwd
    extension:            # Sets file extension fallbacks
      - html
      - htm
    error:                # Custom error page
      404: 404.html
      500: 500.html
    log:                  # Log save location
      error: /logs/error.log
      success: /logs/success.log
# More server ...
```

