use std::fs::{File};
use std::fs;
use std::time::SystemTime;
use std::io::{prelude::*, BufReader};
use url::Url;
use regex::Regex;
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Firewall {
    blacklist: Vec<String>,
    whitelist: Vec<String>,
    // If the operating system can get a modified time, this will be set to true and
    // the list files can be changed while the server is running.
    systime_supported: bool,
    blacklist_last_updated: SystemTime,
    whitelist_last_updated: SystemTime,
}

impl Firewall {

    pub fn new() -> Firewall {
        // Initialize the blacklist and the whitelist.
        let blacklist = Self::update_blacklist();
        let whitelist = Self::update_whitelist();

        // Check if the system supports checking file modification by attempting to obtain it.
        let metadata = fs::metadata("blacklist.txt").unwrap();

        // If this works out, the system does support it. Get the modified timestamps for both lists.
        if let Ok(time) = metadata.modified() {
            let systime_supported = true;
            let blacklist_last_updated = time;
            let metadata = fs::metadata("whitelist.txt").unwrap();
            if let Ok(other_time) = metadata.modified() {
                let whitelist_last_updated = other_time;
                Firewall {
                    blacklist,
                    whitelist,
                    systime_supported,
                    blacklist_last_updated,
                    whitelist_last_updated,
                }
            }
            else {
                // This means the check somehow got the result the first time but didn't work the second time.
                // Default values will have to do.
                let systime_supported = false;
                Firewall {
                    blacklist,
                    whitelist,
                    systime_supported,
                    blacklist_last_updated: SystemTime::now(),
                    whitelist_last_updated: SystemTime::now(),
                }
            }
        }
        // If not, that does away with the possibility of editing on the fly. Dummy data will suffice.
        else {
            let systime_supported = false;
            Firewall {
                blacklist,
                whitelist,
                systime_supported,
                blacklist_last_updated: SystemTime::now(),
                whitelist_last_updated: SystemTime::now(),
            }
        }
    }

    /// Returns true if the given ip is in the blacklist. If supported, also checks if the blacklist has changed and
    /// updates it if necessary.
    pub fn in_blacklist(&mut self, ip: &str) -> bool {
        // Update the blacklist if it's been modified since the last time a request was made.
        if self.systime_supported {
            // Shouldn't need to check if it's OK because the flag covers that.
            let modded = fs::metadata("blacklist.txt").unwrap().modified().unwrap();
            if modded != self.blacklist_last_updated {
                self.blacklist = Self::update_blacklist();
            }
        }

        Self::check_list(self, "blacklist", ip)
    }

    /// Returns true if the given ip is in the whitelist. If supported, also checks if the whitelist has changed and
    /// updates it if necessary.
    pub fn in_whitelist(&mut self, ip: &str) -> bool {
        if self.systime_supported {
            let modded = fs::metadata("whitelist.txt").unwrap().modified().unwrap();
            if modded != self.whitelist_last_updated {
                self.whitelist = Self::update_whitelist();
            }
        }

        Self::check_list(self, "whitelist", ip)
    }

    fn check_list(&self, list: &str, ip: &str) -> bool {
        // For now, wildcards are not supported. This simplifies the checking.
        if list == "whitelist" {
            for list_ip in self.whitelist.iter() {
                if list_ip == ip {
                    return true;
                }
            }
        }
        else if list == "blacklist" {
            for list_ip in self.blacklist.iter() {
                if list_ip == ip {
                    return true;
                }
            }
        }
        
        // The item wasn't found in the given list if we make it this far.
        false
    }

    fn update_blacklist() -> Vec<String> {
        let mut result: Vec<String> = vec!();

        let f = File::open("blacklist.txt").unwrap();
        let r = BufReader::new(f);

        for line in r.lines() {
            result.push(line.unwrap());
        }
        
        result
    }

    fn update_whitelist() -> Vec<String>{
        let mut result: Vec<String> = vec!();

        let f = File::open("whitelist.txt").unwrap();
        let r = BufReader::new(f);

        for line in r.lines() {
            result.push(line.unwrap());
        }
        
        result
    }
}


pub fn malicious_payload(payload: String)->bool{
    let ipv4re = Regex::new(r"^\d{*}.\d{*}.\d{*}.\d{*}$").unwrap();
    if payload.contains("http"){
        Url::parse(payload.into().as_ref()).is_ok();
        return true;
    }else if payload.contains(ipv4re){
        IpAddr::from_str(payload.into().as_ref()).map_or(false, |i| i.is_ipv4());
        return true;
    }return false;
}
