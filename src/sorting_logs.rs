use std::net::{TcpListener, SocketAddr};
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Write, Error};

use std::fs;

//log.txt is the log file generated
const FILE_NAME: &str = "./log.txt";


#[derive(Debug)]
struct Logs {
    file_addr: String,
    date_time: String,
    concatenatedAddrAndDateTime: String,
    
}


// This function is used to sort the log file entries ordered by source and chronology. 
// Summary statistics like types of requests, number of failured/errors will be reported in Deliverable 2 as we develop further features.

 fn sort_logs()->Vec<Logs> {
    
    let content = fs::read_to_string(FILE_NAME).unwrap();
    let mut entries: Vec<Logs> = Vec::new();
    for line in content.lines(){
        let logEntry:Vec<&str>= line.split("|").collect();
       
        let file_addr= logEntry[0].to_string();
        let date_time= logEntry[1].to_string();


        let concatenatedAddrAndDateTime= format!("{} {} ",file_addr , date_time);
       
        entries.push(Logs{
            file_addr,
            date_time,
            concatenatedAddrAndDateTime,
            }
        );
        
    }

    entries.sort_by(|entry_a, entry_b| entry_a.concatenatedAddrAndDateTime.cmp(&entry_b.concatenatedAddrAndDateTime) );
    entries
}


//This function writes the sorted log file.
 fn sorted_log_file(entries:Vec<Logs>) -> Result <(), Error> {
   
    let sorted_log_file = File::open("sortedLogs.txt");

    match sorted_log_file {
        Ok(file) => file,
       
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("sortedLogs.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the sorted log file. Reason: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the sorted log file. Reason: {:?}", other_error);
            },
        },
    };

    let mut sorted_log_file = OpenOptions::new()
    .write(true)
    .append(true)
    .open("sortedLogs.txt")
    .unwrap();

    for entry in entries{
            write!(sorted_log_file, "{} | {}\n", entry.file_addr.to_string(), entry.date_time.to_string())?;      
    }
    Ok(())
}


fn main(){
    let entries= sort_logs();
    sorted_log_file(entries);


}

