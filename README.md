
# dusk

dusk is a static web server, developed using rust.

## Features

## Install

## Config

Use `yaml` format as a configuration file, You can use `dusk -c /your/config.yml` to specify the configuration file location.

Complete configuration file example: 

```yaml
- server:
    host: domain.com   # Domain name to be bound
    listen: 80         # Port to be monitored
    root: /root/www    # Directory that requires service
    gzip: true         # Whether to open Gzip
    index: index.html  # Index file
    directory: true    # Whether to display the file list
    headers:            # Header in response
      - Set-Cookie 12345
      - auth 12345
    extensions:         # Sets file extension fallbacks
      - html
      - htm
    log:               # Log save location
      error: /logs/domain.error.log
      success: /logs/domain.success.log
# More server ...
```

## Todo

* [x] Custom header
* [ ] extension
* [ ] Log
* [ ] Gzip
* [ ] Proxy
* [ ] HTTPS