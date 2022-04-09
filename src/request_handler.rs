
use reqwest::RequestBuilder;

#[derive(Debug, PartialEq)]
pub enum HTTPReqKind {
    GET,
    POST,
    CONNECT,
}

#[derive(Debug)]
pub struct HTTPReq {
    pub req_type: HTTPReqKind,
    pub req_url: String,
}

fn get_req_kind(req: &str) -> Option<HTTPReqKind> {
    
    match req {
        "GET" => Some(HTTPReqKind::GET),
        "CONNECT" => Some(HTTPReqKind::CONNECT),
        "POST" => Some(HTTPReqKind::POST),
        _ => None,
    }
}

pub fn get_req_struct(req: &[u8]) -> () {

    ()
}


pub fn get_http_header(req: &String) -> Vec<&str> {

    let reqs: Vec<&str> = req.split("\r\n").collect();
    let http_header: Vec<&str> = reqs[0].split(" ").collect();

    http_header
}


#[cfg(test)]
mod test_req_handler {
    
    use super::get_http_header;
    use super::get_req_kind;
    use super::get_req_struct;
    use super::HTTPReqKind;

    #[test]
    fn test_req_parsing() {

        let valid_req: String = String::from("CONNECT push.services.mozilla.com:443 HTTP/1.1\r\nUser-Agent: Mozilla/5.0\r\n");

        let res = get_http_header(&valid_req);
        println!("res is {:?}", res);
        assert_eq!("CONNECT", res[0]);
    }

    #[test]
    fn test_req_kind_parsing() {

        let get_valid = "GET";
        let post_valid = "POST";

        println!("Checking {}", get_valid);
        assert_eq!(get_req_kind(get_valid), Some(HTTPReqKind::GET));
    }

    #[test]
    fn test_req_struct() {
        
        let valid_req = b"CONNECT push.services.mozilla.com:443 HTTP/1.1\r\nUser-Agent: Mozilla/5.0\r\n\r\n";

        let res = get_req_struct(valid_req);
        println!("res is {:?}", res);
    }

}
