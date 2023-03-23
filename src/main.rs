use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use select::document::Document;
use select::predicate::Name;

#[tokio::main]
async fn main() {
    println!("=======================");
    println!("Welcome to OctoScraper!");
    println!("=======================");
    let links_map_arc: Arc<Mutex<HashMap<String, i64>>> = Arc::new(Mutex::new(HashMap::new()));
    let id_map_arc: Arc<Mutex<i64>> = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 { 
        let links_map_arc= links_map_arc.clone();
        let id_map_arc = id_map_arc.clone();
        let handle = tokio::spawn(async move {

            println!("{}", id_map_arc.lock().unwrap());
            let links: Vec<String> =  self::extract_links().await;

            
            links.iter().for_each(|item| { 
                
                let mut value = 0;
                if links_map_arc.lock().unwrap().get(item).is_some(){
                    value =  links_map_arc.lock().unwrap().get(item).unwrap() + 1;
                }
                
                links_map_arc.lock().unwrap().insert(item.to_string(), value); });
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }

    links_map_arc.lock().unwrap().iter().for_each(|item| println!("{}|{}", item.0.to_owned(), item.1));
    println!("=======================");
    println!("Done!");
    println!("=======================");
}


async fn extract_links()->Vec<String>{
    let res = reqwest::get("https://www.rust-lang.org/en-US/")
    .await.unwrap()
    .text()
    .await.unwrap();

  let mut data : Vec<String> = Document::from(res.as_str())
    .find(Name("a"))
    .filter_map(|n| n.attr("href")).map(|item| item.to_string()).collect();
    return data;
}