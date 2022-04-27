use publicsuffix::List;

pub fn malicious_url(payload: &String)->bool{
    
    if check_url(payload) || check_host(payload) || check_str(payload) {
        println!("{}", true);
        return true;
    }
    println!("{}", false);
    return false;
}

pub fn check_url(payload: &String)->bool{
    let list = List::fetch().unwrap();
    let mut url = payload.as_str();
    let items = url.split(".");
    println!("Case1:");
    for i in items.into_iter() {
        println!("label Name: {}", i);
        println!("label Len : {}", i.len());
    }
    let domain = list.parse_url(url);
            match domain {
                Err(_) =>  false,
                Ok(result) => true
    }
}

pub fn check_host(payload: &String)->bool{
    let list = List::fetch().unwrap();
    let mut url = payload.as_str();
    let items = url.split(".");
    println!("Case1:");
    for i in items.into_iter() {
        println!("label Name: {}", i);
        println!("label Len : {}", i.len());
    }
    let domain = list.parse_host(url);
            match domain {
                Err(_) =>  false,
                Ok(result) => true
    }

}

pub fn check_str(payload: &String)->bool{
    let list = List::fetch().unwrap();
    let mut url = payload.as_str();
    
    let domain = list.parse_str(url);
            match domain {
                Err(_) =>  false,
                Ok(result) => true
    }

}


