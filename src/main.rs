mod libs;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;




#[derive(Serialize, Deserialize, Debug)]
struct APIKey {
    api_key: String,
}

#[tokio::main]
async fn main() {

    let mut icao: String;
    let api_key: String;
    let mut url: String;
    let metar: String;
    let station: String;

    /* Get API key */
    api_key = read_api_key();

    /* Determine ICAO code for target airport */
    icao = "EDTY".to_string();

    
    /* Determine station name */
    url = create_station_url(&icao, &api_key);
    station = libs::station::read_station_info(&icao).await;

    /* Determine metar code for station */
    url = create_metar_url(&icao, &api_key);
    metar = libs::metar::read_metar_text(&icao).await;

    /* Print result */
    println!("The current METAR for {} is {}", station, metar);


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


fn create_metar_url(icao: &String, api_key: &String) -> String {
    
    let url: String = "https://api.checkwx.com/metar/".to_string() + &icao + "?x-api-key=" + &api_key;

    return url;
}

fn create_station_url(icao: &String, api_key: &String) -> String {

    let url: String = "https://api.checkwx.com/station/".to_string() + &icao + "?x-api-key=" + &api_key;

    return url;
}