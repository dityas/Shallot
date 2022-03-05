
pub enum HTTPReqKind {
    GET,
    POST,
    CONNECT,
}

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

pub fn get_req_struct(req: Option<String>) -> Option<HTTPReq> {
    
    match req {

        Some(req) => parse_request(&req),
        None => (),
    };

    None
}


pub fn parse_request(req: &String) {

    let reqs: Vec<&str> = req.split("\r\n").collect();
    println!("Got {:?}", reqs);
}


#[cfg(test)]
mod test_req_handler {
    
    use super::parse_request;

    #[test]
    fn test_req_parsing() {

        let valid_req: String = String::from("CONNECT push.services.mozilla.com:443 HTTP/1.1\r\nUser-Agent: Mozilla/5.0\r\n");

        parse_request(&valid_req);
    }

}
