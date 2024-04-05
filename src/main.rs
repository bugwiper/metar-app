mod libs;


#[tokio::main]
async fn main() {

    let icao: String;
    let metar: String;
    let station: String;
    let wind: String;


    /* Determine ICAO code for target airport */
    icao = "EDTY".to_string();

    
    /* Determine station name */
    
    station = libs::station::read_station_info(&icao).await;

    /* Determine metar for station */
    metar = libs::metar::read_metar_text(&icao).await;

    /* Determine wind for station */
    wind = libs::metar::get_wind(&icao).await;


    /* Print result */
    println!("The current METAR for {} is {}", station, metar);
    println!("");
    println!("The wind speed is {}kph!", wind);


}








