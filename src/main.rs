mod libs;

use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {

    let mut icao: String;
    let api_key: String;
    let mut url: String;
    let metar: String;
    let station: String;


    /* Determine ICAO code for target airport */
    icao = "EDTY".to_string();

    
    /* Determine station name */
    
    station = libs::station::read_station_info(&icao).await;

    /* Determine metar code for station */
    metar = libs::metar::read_metar_text(&icao).await;

    /* Print result */
    println!("The current METAR for {} is {}", station, metar);


}








