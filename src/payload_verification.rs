use publicsuffix::List;
use regex::Regex;
use crate::logging;
use crate::logging::Event;

const BLACKLIST_DOMAIN_SUFFIX: [&str; 4] = ["in", "pk", "xyz", "wtf"];

pub type Result<T> = std::result::Result<T, PayloadError>;

#[derive(Debug)]
pub enum PayloadError {
    NonHTTPUrl,
    Internal,
    InvalidHost,
    MultipleHosts,
    NoHostFound
}

pub fn check_http_url(buf: &[u8]) -> Result<bool> {
    let http_re = Regex::new(r"^[A-Z]+ .* HTTP/[\d]+\.[\d]+");
    match http_re {
        Ok(re) => {
            let http_header = String::from_utf8_lossy(buf).to_string();

            if re.is_match(&http_header) {
                return Ok(true);
            }

            return Err(PayloadError::NonHTTPUrl);
        },

        Err(_) => Err(PayloadError::Internal)
    }
}

fn verify_host_domain(hst: &str) -> bool {
    let list = List::fetch();
    let host = match hst.find(":") {
        Some(i) => {
            &hst[0..i]
        },
        None => hst
    };

    match list {
        Ok(list) => {
            let domain = list.parse_domain(host);

            return match domain {
                Ok(domain) => {
                    if !domain.is_icann() {
                        return false;
                    }
                    match domain.suffix() {
                        Some(suffix) => {
                            !BLACKLIST_DOMAIN_SUFFIX.contains(&suffix)
                        },
                        None => {
                            true
                        }
                    }
                },
                Err(_) => {
                    true
                }
            }
        },
        Err(_) => {
            return true;
        }
    }
}

pub fn check_host(buf: &[u8]) -> Result<bool> {

    let host_re = Regex::new(r"Host: (.*)");

    return match host_re {
        Ok(host_re) => {
            let http_header = String::from_utf8_lossy(buf).to_string();

            let host_count = host_re.captures_iter(&http_header).count();

            if host_count != 1 {
                return if host_count == 0 {
                    Err(PayloadError::NoHostFound)
                } else {
                    Err(PayloadError::MultipleHosts)
                }
            }

            for cap_host in host_re.captures_iter(&http_header) {
                if !verify_host_domain(&cap_host[1]) {
                    return Err(PayloadError::InvalidHost);
                }
            }

            Ok(true)
        },
        Err(_) => {
            Err(PayloadError::Internal)
        }
    }
}
