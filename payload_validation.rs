use std::fs::File;
use std::borrow::Cow;
use url::Url;
use regex::Regex;
use std::net::IpAddr;
use std::str::FromStr;
use std::io::prelude::*;

fn malicious_payload(payload: String)->bool{
    let ipv4re = Regex::new(r"^\d{*}.\d{*}.\d{*}.\d{*}$").unwrap();
    if payload.contains("http"){
        Url::parse(payload.into().as_ref()).is_ok();
        return true;
    }else if payload.contains(ipv4re){
        IpAddr::from_str(payload.into().as_ref()).map_or(false, |i| i.is_ipv4());
        return true;
    }return false;
}

/*
fn malicious() -> std::io::Result<()> {
    let mut file = File::open("samplepayload.txt")?;
    let mut payload = String::new();
    file.read_to_string(&mut payload).unwrap();

    Ok(())
}*/
/*
#[must_use]
pub fn validate_url<'a, payload>(val: payload) -> bool
where
    payload: Into<Cow<'a, str>>,
{
    Url::parse(val.into().as_ref()).is_ok()
}
#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::validate_url;

    #[test]
    fn test_validate_url() {
        let tests = vec![
            ("http", false),
            ("https://google.com", true),
            ("http://localhost:80", true),
            ("ftp://localhost:80", true),
        ];

        for (url, expected) in tests {
            assert_eq!(validate_url(url), expected);
        }
    }

    #[test]
    fn test_validate_url_cow() {
        let test: Cow<'static, str> = "http://localhost:80".into();
        assert_eq!(validate_url(test), true);
        let test: Cow<'static, str> = String::from("http://localhost:80").into();
        assert_eq!(validate_url(test), true);
        let test: Cow<'static, str> = "http".into();
        assert_eq!(validate_url(test), false);
        let test: Cow<'static, str> = String::from("http").into();
        assert_eq!(validate_url(test), false);
    }
}

#[must_use]
pub fn validate_ip_v4<'a, payload>(val: payload) -> bool
where
    payload: Into<Cow<'a, str>>,
{
    IpAddr::from_str(val.into().as_ref()).map_or(false, |i| i.is_ipv4())
}

/// Validates whether the given string is an IP V6
#[must_use]
pub fn validate_ip_v6<'a, payload>(val: payload) -> bool
where
    payload: Into<Cow<'a, str>>,
{
    IpAddr::from_str(val.into().as_ref()).map_or(false, |i| i.is_ipv6())
}



#[cfg(test)]
mod tests1 {
    use std::borrow::Cow;

    use super::{ validate_ip_v4, validate_ip_v6};


    #[test]
    fn test_validate_ip_v4() {
        let tests = vec![
            ("1.1.1.1", true),
            ("255.0.0.0", true),
            ("0.0.0.0", true),
            ("256.1.1.1", false),
            ("25.1.1.", false),
            ("25,1,1,1", false),
        ];

        for (input, expected) in tests {
            assert_eq!(validate_ip_v4(input), expected);
        }
    }

    #[test]
    fn test_validate_ip_v6() {
        let tests = vec![
            ("fe80::223:6cff:fe8a:2e8a", true),
            ("2a02::223:6cff:fe8a:2e8a", true),
            ("1::2:3:4:5:6:7", true),
            ("::", true),
            ("::a", true),
            ("2::", true),
            ("::ffff:254.42.16.14", true),
            ("::ffff:0a0a:0a0a", true),
        ];

        for (input, expected) in tests {
            assert_eq!(validate_ip_v6(input), expected);
        }
    }
}*/
/*

Cargo toml
make sure to add
Dependencies
url = "2"
serde = "1"

*/
