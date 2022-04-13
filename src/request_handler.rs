
use hyper::Body;
use hyper::http::{Request, Response};

use std::convert::Infallible;

use crate::logging;

pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {

    logging::event_log(format!("[Request event] Got: {:?}", req).as_str());
    Ok(Response::new(Body::from("Hi from Proxy!")))
}


#[cfg(test)]
mod test_req_handler {
    

}
