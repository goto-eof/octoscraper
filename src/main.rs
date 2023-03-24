use crate::page_processor::extract_links_and_process_data;
use std::collections::HashSet;

mod page_processor;

#[tokio::main]
async fn main() {
    println!("====================================================================");
    println!("Welcome to");
    println!(
        r#"
    ___     _        __                                
    /___\___| |_ ___ / _\ ___ _ __ __ _ _ __   ___ _ __ 
   //  // __| __/ _ \\ \ / __| '__/ _` | '_ \ / _ \ '__|
  / \_// (__| || (_) |\ \ (__| | | (_| | |_) |  __/ |   
  \___/ \___|\__\___/\__/\___|_|  \__,_| .__/ \___|_|   
                                       |_|              
    "#,
    );
    println!("====================================================================");
    let website = "http://dodu.it";
    let domain = website.clone();
    let mut processing: HashSet<String> = HashSet::new();
    let mut processed: HashSet<String> = HashSet::new();
    processing.insert(website.to_string());
    while processing.len() > 0 {
        let link = processing.clone();
        let link = link.iter().next().unwrap();
        processing.remove(link.as_str());
        extract_links_and_process_data(link, domain, &mut processing, &mut processed).await;
    }
}
