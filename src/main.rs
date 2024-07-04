mod libs;
use std::io;
use libs::airport::Airport;

use crate::libs::airport;


#[tokio::main]
async fn main() {

    let mut airports: Vec<Airport> = Vec::new();
    let icaos: Vec<String> = vec![
        "EDTY".to_string(),
        "EDDS".to_string(),
        "EDDF".to_string(),
        "EDDM".to_string(),
        "EDDB".to_string(),
    ];

    /* Create airports and their METAR codes based on ICAO airport identifier */

    for x in icaos.iter() {
        let mut airport = Airport::new(&x);
        airport = airport::update(airport).await;
        // TODO: no internet connection
        airports.push(airport);
    }
    println!("METAR list:");
    for x in airports.iter(){
        println!("{},", x.metar);
    }

    //debug_print(airport);

}


fn debug_print(input: airport::Airport){
    
    println!("\nDebug airport:");
    println!("{}", input.icao);
    println!("{}:{}z", input.utc_hour, input.utc_minute);
    println!("Q{}", input.qnh);
    println!("w{}/{}",input.wind_direction, input.wind_speed);
    println!("CAVOK? {}", input.cavok);
    println!("Confidence? {}", input.confidence);
    println!("temp {}/{}", input.temperature, input.dewpoint);
    println!("vis {}", input.visibility);
    for elem in input.clouds{
        println!("cloud: {} in {}", elem.class, elem.base);
    }

}








