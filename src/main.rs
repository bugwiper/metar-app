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

    println!("Determine airport ICAO code...");
    icao = "EDTY".to_string();
    println!("Code: {}", &icao);

    
    url = create_url(&icao, &api_key);

    let reply = read_metar_text(&url).await;

    let res: String = match reply {
        Ok(v) => v,
        Err(_e) => return,
      };

    // Convert into JSON object

    let object: MetarData = serde_json::from_str(&res).unwrap();

    metar = object.data[0].to_string();
    println!("This is the METAR: {}", metar);


}

async fn read_metar_text(input: &String) -> Result <String, reqwest::Error> {

    println!("Read metar in text...");
    // Perform the HTTP request
    let response =  reqwest::get(input)
        .await?
        .text()
        .await?;  

    return Ok(response);
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
        Ok(_) => print!("API-key from file: "),
    }

    //Convert into JSON object

    let object: APIKey = serde_json::from_str(&s).unwrap();

    let key = object.api_key;
    println!("{}", &key);

    return key;

    // `file` goes out of scope, and the "hello.txt" file gets closed
}


fn create_url(icao: &String, api_key: &String) -> String {
    
    let mut url: String = "https://api.checkwx.com/metar/".to_string() + &icao + "?x-api-key=" + &api_key;

    return url;
}