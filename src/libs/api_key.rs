use serde::Deserialize; 
use serde::Serialize;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct APIKey {
    api_key: String,
}


pub fn read_api_key() -> String{
    
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