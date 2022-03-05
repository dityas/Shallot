
#[derive(Debug, PartialEq)]
pub enum HTTPReqKind {
    GET,
    POST,
    CONNECT,
}

#[derive(Debug)]
pub struct HTTPReq {
    req_type: HTTPReqKind,
    req_url: String,
}

pub fn convert_to_string(byte_array: &[u8]) -> Option<String> {

    match String::from_utf8(byte_array.to_vec()) {
        Ok(the_string) => Some(the_string),
        Err(err) => None,
    }
}

fn get_req_kind(req: &str) -> Option<HTTPReqKind> {
    
    match req {
        "GET" => Some(HTTPReqKind::GET),
        "CONNECT" => Some(HTTPReqKind::CONNECT),
        "POST" => Some(HTTPReqKind::POST),
        _ => None,
    }
}

pub fn get_req_struct(req: &String) -> Option<HTTPReq> {
    
    let res = get_http_header(req);
    let kind = get_req_kind(res[0])?;

    Some(HTTPReq {
        req_type: kind,
        req_url: String::from(res[1]),
    })
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
        
        let valid_req: String = String::from("CONNECT push.services.mozilla.com:443 HTTP/1.1\r\nUser-Agent: Mozilla/5.0\r\n");

        let res = get_req_struct(&valid_req);
        println!("res is {:?}", res);
    }

}
