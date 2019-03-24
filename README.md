
# sws [![Build Status](https://img.shields.io/travis/wyhaya/sws.svg?style=flat-square)](https://travis-ci.org/wyhaya/sws)

sws is a static web server, developed using rust.

## Features

## Use

```bash
cargo +nightly run
```

## Config

Use `yaml` format as a configuration file, You can use `sws -c /your/config.yml` to specify the configuration file location.

Complete configuration file example: 

```yaml
- server:
    host: domain.com      # Domain name to be bound
    listen: 80            # Port to be monitored
    root: /root/www       # Directory that requires service
    index: index.html     # Index file
    directory: true       # Whether to display the file list
    headers:              # Header in response
      Access-Control-Allow-Origin: *
      Set-Cookie: 12345
    gzip:                 # Whether to open Gzip
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

* [x] Custom header
* [x] Extensions
* [ ] Parse request `50%`
* [x] Custom error
* [x] Async/response
* [ ] Async/log `20%`
* [x] HTTP Auth
* [ ] Gzip
* [ ] Proxy
* [ ] HTTPS / HTTP2