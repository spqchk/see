

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


const LINE: [u8; 2] = [13, 10];          // '\r\n'
const SPACE: [u8; 1] = [32];             // ' '
const COLON_SPACE: [u8; 2] = [58, 32];   // ': '
const QUESTION_MARK: [u8; 1] = [63];     // '?'


impl Request {

    pub fn new(buff: &[u8]) -> Result<Request, ()> {

        let req = Request::split(&buff.to_vec(), &LINE);
        if req.len() < 1 {
            return Err(());
        }

        let (method, path, query) = if let Ok(first) = Request::first(&req[0]) {
            first
        }else {
            return Err(());
        };

        let mut headers = HashMap::new();
        if req.len() > 1 {
            for h in &req[1..] {
                let header = Request::split(h, &COLON_SPACE);
                if header.len() != 2 {
                    break;
                }
                // The key in the header is uniformly used in lowercase
                headers.insert(
                    String::from_utf8_lossy(&header[0]).to_lowercase(),
                    String::from_utf8_lossy(&header[1]).to_string(),
                );
            }
        }

        Ok(Request {
            method,
            path,
            query,
            headers
        })

    }

    fn split(buff: &Vec<u8>, split: &[u8]) -> Vec<Vec<u8>> {

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

    fn first(buff: &Vec<u8>) -> Result<(String, String, Option<String>), ()> {

        let line = Request::split(buff, &SPACE);
        if line.len() != 3 {
            return Err(());
        }
        let method = String::from_utf8_lossy(&line[0]).to_string();
        let url = Request::split(&line[1], &QUESTION_MARK);

        let path = if let Ok(path) = percent_decode(&url[0]).decode_utf8() {
            path.to_string()
        }else {
            return Err(());
        };

        let query = if let Some(q) = url.get(1) {
            match percent_decode(q).decode_utf8() {
                Ok(q) => Some(format!("?{}", q)),
                Err(_) => None
            }
        }else {
            None
        };

        Ok((method, path, query))

    }

}


#[cfg(test)]
mod tests {

    use crate::request::Request;

    #[test]
    fn test_split() {
        assert_eq!(
            Request::split(&vec![], &[1]),
            Vec::<Vec<u8>>::new()
        );
        assert_eq!(
            Request::split(&vec![1, 2], &[1]),
            vec![vec![2]]
        );
        assert_eq!(
            Request::split(&vec![1, 2], &[2]),
            vec![vec![1]]
        );
        assert_eq!(
            Request::split(&vec![1, 2, 3], &[2]),
            vec![vec![1], vec![3]]
        );
        assert_eq!(
            Request::split(&vec![49, 50, 51, 52, 53], &[51]),
            vec![vec![49, 50], vec![52, 53]]
        );
        assert_eq!(
            Request::split(&vec![1, 2, 3, 4, 5, 6], &[1, 2, 3, 4, 5, 6]),
            Vec::<Vec<u8>>::new()
        );
    }

    #[test]
    fn test_parse_first() {
        assert_eq!(
            Request::first(&b"".to_vec()),
            Err(())
        );
        assert_eq!(
            Request::first(&b"POST /abc".to_vec()),
            Err(())
        );
        assert_eq!(
            Request::first(&b"POST /abc HTTP/1.1".to_vec()),
            Ok((
                String::from("POST"),
                String::from("/abc"),
                None
            ))
        );
        assert_eq!(
            Request::first(&b"GET /abc?type=1 HTTP/1.1".to_vec()),
            Ok((
                String::from("GET"),
                String::from("/abc"),
                Some(String::from("?type=1"))
            ))
        );
    }

    #[test]
    fn test_parse_request() {
        let buff = b"\
            GET /abc?type=1 HTTP/1.1\r\n\
            Host: 127.0.0.1\r\n\
            Accept-Encoding: gzip, deflate, br\r\n\
            \r\n\r\n\
            hello world\
        ";
        let req = Request::new(buff).unwrap();
        assert_eq!(&req.method, "GET");
        assert_eq!(&req.path, "/abc");
        assert_eq!(req.headers.get("host").unwrap(), "127.0.0.1");
        assert_eq!(req.headers.get("accept-encoding").unwrap(), "gzip, deflate, br");
    }

}



