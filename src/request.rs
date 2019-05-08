

extern crate percent_encoding;
use std::collections::HashMap;
use percent_encoding::percent_decode;


#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub query: Option<String>,
    pub headers: HashMap<String, String>
}


fn split(buff: &Vec<u8>, split: Vec<u8>) -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = vec![];
    let (buff_len, split_len) = (buff.len(), split.len());
    let mut find = 0;
    for (i, _) in buff.iter().enumerate() {
        if buff_len - split_len >= i {
            let cur = buff[i..i + split_len].to_vec();
            if cur == split {
                if find != i {
                    result.push(buff[find..i].to_vec());
                }
                find = i + split_len;
            }
        }
    }
    if find != buff_len {
        result.push(buff[find..].to_vec());
    }
    result
}


impl Request {

    pub fn new(buff: &[u8]) -> Request {

        let req = split(&buff.to_vec(), vec![13, 10]);
        let line = split(&req[0], vec![32]);
        let method = String::from_utf8_lossy(&line[0]).to_string();
        let full_path = split(&line[1], vec![63]);

        let path = percent_decode(&full_path[0])
            .decode_utf8()
            .unwrap()
            .to_string();
       
        let query = if let Some(q) = full_path.get(1) {
            Some(percent_decode(q)
                .decode_utf8()
                .unwrap()
                .to_string())
        }else {
            None
        };

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
            query,
            headers
        }

    }

}


#[cfg(test)]
mod tests {

    use crate::request::split;
    use crate::request::Request;

    #[test]
    fn test_split() {
        assert_eq!(
            split(&vec![1, 2], vec![2]),
            vec![vec![1]]
        );
        assert_eq!(
            split(&vec![1, 2], vec![1]),
            vec![vec![2]]
        );
        assert_eq!(
            split(&vec![1, 2, 3], vec![2]),
            vec![vec![1], vec![3]]
        );
        assert_eq!(
            split(&vec![49, 50, 51, 52, 53], vec![51]),
            vec![vec![49, 50], vec![52, 53]]
        );
        assert_eq!(
            split(&vec![1, 2, 3, 4, 5, 6], vec![1, 2, 3, 4, 5, 6]),
            Vec::<Vec<u8>>::new()
        );
    }

    #[test]
    fn test_parse_request() {
        let buff = b"\
            GET /abc?type=1 HTTP/1.1\r\n\
            Host: 127.0.0.1\r\n\
            Accept-Encoding: gzip, deflate, br\r\n\
            a: 1\
            \r\n\r\n\
            hello world\
        ";
        let req = Request::new(buff);
        assert_eq!(&req.method, "GET");
        assert_eq!(&req.path, "/abc");
        assert_eq!(req.headers.get("host").unwrap().as_str(), "127.0.0.1");
        assert_eq!(req.headers.get("accept-encoding").unwrap().as_str(), "gzip, deflate, br");
        assert_eq!(req.headers.get("a").unwrap().as_str(), "1");
    }

}



