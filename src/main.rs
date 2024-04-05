use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct MetarData {
    results: i32,
    data: [String; 1],
}

#[derive(Serialize, Deserialize, Debug)]
struct APIKey {
    api_key: String,
}

#[tokio::main]
async fn main() {

    let mut icao: String;
    let api_key: String;
    let url: String;
    let metar: String;

    println!("Get API key...");
    api_key = read_api_key();

    print!("Determine Schwäbisch Hall airport ICAO code... ");
    icao = "EDTY".to_string();
    println!("{}", &icao);

    
    url = create_url(&icao, &api_key);

    metar = read_metar_text(&url).await;

    println!("This is the METAR: {}", metar);


}

async fn read_metar_text(input: &String) -> String {

    println!("Read metar in text...");
    // Perform the HTTP request
    let response =  reqwest::get(input)
        .await
        .unwrap()
        .text()
        .await;  

     //let res = response.unwrap();

     let res = match response {
        Ok(v) => v,
        Err(_err) => return "no metar found".to_string(),
     };

    // Convert into JSON object 

    let object: MetarData = serde_json::from_str(&res).unwrap();

    let metar = object.data[0].to_string();

    return metar;
}

fn read_api_key() -> String{
    
    // Create a path to the desired file
    let path = Path::new("src/api_key.json");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }

    //Convert into JSON object

    let object: APIKey = serde_json::from_str(&s).unwrap();

    let key = object.api_key;

    return key;

    // `file` goes out of scope, and the "hello.txt" file gets closed
}


fn create_url(icao: &String, api_key: &String) -> String {
    
    let url: String = "https://api.checkwx.com/metar/".to_string() + &icao + "?x-api-key=" + &api_key;

    return url;
}