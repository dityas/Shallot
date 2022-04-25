use std::fs::File;
use std::fs;
use std::{thread, time};
use std::time::Duration;
use regex::Regex;


pub fn generate_statistics() {
    File::create("./statistics.txt").expect("Unable to create statistics file.");
    let wait_time = time::Duration::from_secs(5);

    loop {
        let mut log = fs::read_to_string("./event_log.txt").expect("Unable to read log.txt");
        let mut whitelist_deny = 0;
        let mut blacklist_deny = 0;
        let mut connection = 0;
        let mut data_transfer = 0;
        let mut proxy_server = 0;
        let mut suspicious_activity = 0;
        let mut uncategorised = 0;

        for log_line in log.split("\n") {
            if log_line.contains("Blacklist Deny") {
                blacklist_deny += 1;
            } else if log_line.contains("Whitelist Deny") {
                whitelist_deny += 1;
            } else if log_line.contains("Connection") {
                connection += 1;
            } else if log_line.contains("Data Transfer") {
                data_transfer += 1;
            } else if log_line.contains("Proxy Server") {
                proxy_server += 1;
            } else if log_line.contains("Suspicious Activity") {
                suspicious_activity += 1;
            } else {
                uncategorised += 1;
            }
        }
        let statistics_text = format!(
            "Total number of connections: {}\n\
            Number of whitelist deny events: {}\n\
            Number of blacklist deny events: {}\n\
            Number of data transfer events: {}\n\
            Number of proxy server events: {}\n\
            Number of suspicious activities events: {}\n\
            Number of uncategorized events: {}",
            connection, whitelist_deny, blacklist_deny, data_transfer,
            proxy_server, suspicious_activity, uncategorised);

        fs::write("./statistics.txt", statistics_text).expect("Unable to write");

        thread::sleep(wait_time);
    }
}