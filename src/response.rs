

#[derive(Default)]
pub struct Response {
    line: String,
    header: String
}

pub enum StatusCode {
    Ok,
    NotFound,
    Error,
    Moved
}

impl Response {

    pub fn new(status: StatusCode) -> Response {

        let mut response = Response::default();

        response.line = match status {
            StatusCode::Ok => String::from("HTTP/1.1 200 OK\r\n"),
            StatusCode::NotFound => String::from("HTTP/1.1 404\r\n"),
            StatusCode::Error => String::from("HTTP/1.1 500\r\n"),
            StatusCode::Moved => String::from("HTTP/1.1 301\r\n")
        };

        response.header += "Server: dusk\r\n";
        response

    }

    pub fn header(mut self, key: &str, value:  &str) -> Response {

        self.header += key;
        self.header += ": ";
        self.header += value;
        self.header += "\r\n";

        Response {
            ..self
        }

    }

    pub fn headers(mut self, headers: &Vec<Vec<String>>) -> Response {

        for header in headers {
            self.header += header[0].as_str();
            self.header += ": ";
            self.header += header[1].as_str();
            self.header += "\r\n";
        }

        Response {
            ..self
        }

    }


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

    pub fn body(self, data: &[u8]) -> Vec<u8> {

        let res = self.header("Content-Length", &data.len().to_string());

        let mut top = String::from("");
        top.push_str(&res.line);
        top.push_str(&res.header);
        top.push_str("\r\n");

        [&top.as_bytes()[..], &data[..]].concat()

    }


}

