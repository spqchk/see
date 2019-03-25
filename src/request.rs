

use std::collections::HashMap;
extern crate percent_encoding;
use percent_encoding::percent_decode;


#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>
}


fn split(buff: &Vec<u8>, split: Vec<u8>) -> Vec<Vec<u8>> {
    let mut req: Vec<Vec<u8>> = vec![];
    let buff_size = buff.len();
    let size = split.len();
    let mut find = 0;
    for (i, _) in buff.iter().enumerate() {
        if i + size + 1 < buff_size {
            let cur = &buff[i..i + size];
            if cur == &split[..] {
                let b: Vec<u8> = buff[find..i].iter().cloned().collect();
                req.push(b);
                find = i + size;
            }
        }
    }
    let end: Vec<u8> = buff[find..].iter().cloned().collect();
    req.push(end);
    req
}

impl Request {

    pub fn new(buff: &[u8]) -> Request {

        let buff: Vec<u8> = buff.iter().cloned().collect();
        let req = split(&buff, vec![13, 10]);
        let line = split(&req[0], vec![32]);
        let method = String::from_utf8_lossy(&line[0]).to_string();
        let full_path = split(&line[1], vec![63]);
        let path = percent_decode(&full_path[0])
            .decode_utf8()
            .unwrap()
            .to_string();

        let mut headers = HashMap::new();
        for h in &req[1..] {
            if h.len() == 0 {
                continue;
            }
            let header = split(h, vec![58, 32]);
            if header.len() != 2 {
                continue;
            }
            // The key in the header is uniformly used in lowercase
            headers.insert(
                String::from_utf8_lossy(&header[0]).to_lowercase(),
                String::from_utf8_lossy(&header[1]).to_string(),
            );
        }

        Request {
            method,
            path,
            headers
        }

    }

}


#[test]
fn test_parse_request() {
    let buff = b"GET /abc?type=1 HTTP/1.1\r\nHost: 127.0.0.1\r\na: 1\r\n\r\n";
    let req = Request::new(buff);
    assert_eq!(&req.method, "GET");
    assert_eq!(&req.path, "/abc");
    assert_eq!(req.headers.get("host").unwrap().as_str(), "127.0.0.1");
//    assert_eq!(req.headers.get("a").unwrap().as_str(), "1");
}



