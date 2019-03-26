

extern crate libflate;
use crate::config::Header;
use std::io::Write;
use libflate::gzip;
use libflate::deflate;
use libflate::zlib;

#[derive(Default)]
pub struct Response {
    line: String,
    header: String,
    compress: Vec<Compress>
}

pub enum StatusCode {
    _200,
    _301,
    _400,
    _401,
    _404,
    _405,
    _500
}

#[derive(Debug)]
pub enum Compress {
    Gzip,
    Deflate,
    Br
}

impl Response {

    // HTTP response
    pub fn new(status: StatusCode) -> Response {

        let mut response = Response::default();

        response.line = match status {
            StatusCode::_200 => String::from("HTTP/1.1 200 OK\r\n"),
            StatusCode::_400 => String::from("HTTP/1.1 400\r\n"),
            StatusCode::_301 => String::from("HTTP/1.1 301\r\n"),
            StatusCode::_401 => String::from("HTTP/1.1 401\r\n"),
            StatusCode::_404 => String::from("HTTP/1.1 404\r\n"),
            StatusCode::_405 => String::from("HTTP/1.1 405\r\n"),
            StatusCode::_500 => String::from("HTTP/1.1 500\r\n")
        };

        // Add service name
        response.header += "Server: sws\r\n";
        response

    }

    // Set header
    pub fn header(mut self, key: &str, value:  &str) -> Response {

        self.header += key;
        self.header += ": ";
        self.header += value;
        self.header += "\r\n";

        Response {
            ..self
        }

    }

    // Set multiple header
    pub fn headers(mut self, headers: &Vec<Header>) -> Response {

        for header in headers {
            self.header += &header.key;
            self.header += ": ";
            self.header += &header.value;
            self.header += "\r\n";
        }

        Response {
            ..self
        }

    }

    // Set the content-type based on the file extension
    pub fn content_type(self, ext: &str) -> Response {

        let value = match &ext.as_ref() {
            &"aac" => "audio/aac",
            &"abw" => "application/x-abiword",
            &"arc" => "application/x-freearc",
            &"avi" => "video/x-msvideo",
            &"azw" => "application/vnd.amazon.ebook",
            &"bin" => "application/octet-stream",
            &"bmp" => "image/bmp",
            &"bz" => "application/x-bzip",
            &"bz2" => "application/x-bzip2",
            &"csh" => "application/x-csh",
            &"css" => "text/css",
            &"csv" => "text/csv",
            &"doc" => "application/msword",
            &"docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            &"eot" => "application/vnd.ms-fontobject",
            &"epub" => "application/epub+zip",
            &"gif" => "image/gif",
            &"htm" => "text/html",
            &"html" => "text/html",
            &"ico" => "image/vnd.microsoft.icon",
            &"ics" => "text/calendar",
            &"jar" => "application/java-archive",
            &"jpeg" => "image/jpeg",
            &"jpg" => "image/jpeg",
            &"js" => "text/javascript",
            &"json" => "application/json",
            &"mjs" => "text/javascript",
            &"mp3" => "audio/mpeg",
            &"mpeg" => "video/mpeg",
            &"mpkg" => "application/vnd.apple.installer+xml",
            &"odp" => "application/vnd.oasis.opendocument.presentation",
            &"ods" => "application/vnd.oasis.opendocument.spreadsheet",
            &"odt" => "application/vnd.oasis.opendocument.text",
            &"oga" => "audio/ogg",
            &"ogv" => "video/ogg",
            &"ogx" => "application/ogg",
            &"otf" => "font/otf",
            &"png" => "image/png",
            &"pdf" => "application/pdf",
            &"ppt" => "application/vnd.ms-powerpoint",
            &"pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            &"rar" => "application/x-rar-compressed",
            &"rtf" => "application/rtf",
            &"sh" => "application/x-sh",
            &"svg" => "image/svg+xml",
            &"swf" => "application/x-shockwave-flash",
            &"tar" => "application/x-tar",
            &"tif" => "image/tiff",
            &"tiff" => "image/tiff",
            &"ttf" => "font/ttf",
            &"txt" => "text/plain",
            &"vsd" => "application/vnd.visio",
            &"wav" => "audio/wav",
            &"weba" => "audio/webm",
            &"webm" => "video/webm",
            &"webp" => "image/webp",
            &"woff" => "font/woff",
            &"woff2" => "font/woff2",
            &"xhtml" => "application/xhtml+xml",
            &"xls" => "application/vnd.ms-excel",
            &"xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            &"xml" => "text/xml",  // application/xml
            &"xul" => "application/vnd.mozilla.xul+xml",
            &"zip" => "application/zip",
            &"3gp" => "video/3gpp",  // audio/video
            &"3g2" => "video/3gpp2",  // audio/3gpp2
            &"7z" => "application/x-7z-compressed",
            _ => "application/octet-stream"
        };

        self.header("Content-Type", value)

    }

    pub fn compress(self, way: Vec<Compress>) -> Response {
        Response {
            compress: way,
            ..self
        }
    }

    // Build a complete response
    pub fn body(self, data: &[u8]) -> Vec<u8> {

        let res = self.header("Content-Length", &data.len().to_string());

        let mut top = String::from("");
        top.push_str(&res.line);

        if &res.compress.len() == &0 {
            top.push_str(&res.header);
            top.push_str("\r\n");
            return [&top.as_bytes()[..], &data[..]].concat();
        }

        let (min_data, way) = min(data, &res.compress);
        match way {
            Compress::Gzip => {
                let d = &res.header("Content-Encoding", "gzip");
                top.push_str(&d.header);
                top.push_str("\r\n");
                return [&top.as_bytes()[..], &min_data[..]].concat();
            },
            Compress::Deflate => {
                let d = &res.header("Content-Encoding", "deflate");
                top.push_str(&d.header);
                top.push_str("\r\n");
                return [&top.as_bytes()[..], &min_data[..]].concat();
            },
            _ => {

            }
        }
        [&top.as_bytes()[..], &min_data[..]].concat()
    }

}

fn min(data: &[u8], compress: &Vec<Compress>) -> (Vec<u8>, Compress) {
    for way in compress {
        match way {
            Compress::Gzip => {
                return (gzip(data), Compress::Gzip);
            },
            Compress::Deflate => {
                return (deflate(data), Compress::Deflate);
            },
            Compress::Br => {
                break;
            }
        }
    }
    return (gzip(data), Compress::Gzip);
}

fn gzip(data: &[u8]) -> Vec<u8> {
    let mut encoder = gzip::Encoder::new(Vec::new()).unwrap();
    encoder.write_all(data).unwrap();
    encoder.finish().into_result().unwrap()
}

fn deflate(data: &[u8]) -> Vec<u8> {
    let mut encoder = deflate::Encoder::new(Vec::new());
    encoder.write_all(&data).unwrap();
    encoder.finish().into_result().unwrap()
}

