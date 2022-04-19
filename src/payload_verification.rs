use publicsuffix::List;
use regex::Regex;

pub fn verify_host(host: String) -> bool {
    if host.starts_with("localhost") {
        println!("Skipping verify host check for local host.");
        return true;
    }
    let list = List::fetch().unwrap();
    let domain = list.parse_domain(&host).unwrap();

    if !domain.is_icann() {
        println!("Is not icann");
        return false;
    }
    if domain.is_private() {
        println!("Is private");
        return false;
    }

    return true;
}

pub fn verify_payload(payload: &str) -> bool {
    const THRESHOLD: usize = 10000;
    // Check of large payloads
    if payload.len() > THRESHOLD {
        return false;
    }
    return true;
}

pub fn verify_http(payload: &str) -> bool {
    let http_re = Regex::new(r"^[A-Z]+ /.* HTTP/[\d]+\.[\d]+$").unwrap();
    return !http_re.is_match(payload);
}
