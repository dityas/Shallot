// Dependencies:
// chrono = "0.4"

// TcpListener can be removed once the main function is also removed.
use chrono::{DateTime, Local};
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Write};
use std::net::{SocketAddr, TcpListener};

pub enum Event {
    WhiteListDeny,
    BlackListDeny,
    Connection,
    DataTransfer,
    ProxyServer,
    SuspiciousActivity,
    Uncategorized,
}

pub fn log(addr: SocketAddr, connection_result: &str) -> Result<(), Error> {
    // Checking to see if the file exists every time seems like overkill. It maybe better to create log.txt on the
    // server side.
    let log_file = File::open("log.txt");

    match log_file {
        Ok(file) => file,
        // This should only need to happen once per place the host happens.
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("log.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating log.txt. Reason: {:?}", e),
            },
            other_error => {
                panic!("Problem opening log.txt. Reason: {:?}", other_error);
            }
        },
    };

    let mut log_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("log.txt")
        .unwrap();

    let time: DateTime<Local> = Local::now();

    // This will very likely need to have more added to it as we develop further features, especially for the second
    // deliverable.
    writeln!(
        log_file,
        "{} | {} | {}",
        addr.to_string(),
        time.to_string(),
        connection_result
    )?;

    Ok(())
}

pub fn event_log(event: Event, msg: &str) -> Result<(), Error> {
    let event_file = File::open("event_log.txt");

    match event_file {
        Ok(file) => file,
        // This should only need to happen once per place the host happens.
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("event_log.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating event_log.txt. Reason: {:?}", e),
            },
            other_error => {
                panic!("Problem opening event_log.txt. Reason: {:?}", other_error);
            }
        },
    };

    let mut event_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("event_log.txt")
        .unwrap();

    let time: DateTime<Local> = Local::now();
    let mut event_msg = String::new();

    match event {
        Event::BlackListDeny => event_msg += "[Blacklist Deny]",
        Event::WhiteListDeny => event_msg += "[Whitelist Deny]",
        Event::Connection => event_msg += "[Connection]",
        Event::DataTransfer => event_msg += "[Data Transfer]",
        Event::ProxyServer => event_msg += "[Proxy Server]",
        Event::SuspiciousActivity => event_msg += "[Suspicious Activity]",
        Event::Uncategorized => event_msg += "[Uncategorized]",
        _ => event_msg += "[Uncategorized]",
    };

    writeln!(
        event_file,
        "{} {}: {}",
        time.format("[%b %d, %Y; %I:%M %p]").to_string(),
        event_msg,
        msg
    )?;

    // For console logging
    println!(
        "{} {}: {}",
        time.format("[%b %d, %Y; %I:%M %p]").to_string(),
        event_msg,
        msg
    );

    Ok(())
}

// Sample running of the log server with single listen. Must be run mulitple times to get multiple log lines.
// Can do away with the warning and this code for deliverable 2.
#[allow(dead_code)]
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    match listener.accept() {
        Ok((_socket, addr)) => {
            println!("new client: {:?}", addr);
            match log(addr, "OK") {
                Err(e) => println!("Uncaught issue with the log function: {:?}", e),
                _ => (),
            };
        }
        Err(e) => println!("couldn't get client {:?}", e),
    }
}
