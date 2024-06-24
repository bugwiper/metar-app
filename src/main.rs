mod libs;
use std::io;
use crate::libs::airport;


#[tokio::main]
async fn main() {

    let temp = "ENBR".to_string();
     

    /* Determine metar code for target airport */

    let mut my_airport = airport::Airport::new(&temp);
    my_airport = airport::update(my_airport).await;
    // TODO: no internet connection
    println!("METAR: {}", my_airport.metar);

    debug_print(my_airport);

}

fn validate_user_input(input: &String) -> bool {

    if input.len() == 4 {
        return true 
    } else {
        return false
    }
}

async fn collect_static_metar_list() {

    println!{"Preparing static metar list."};

    let mut icao_list: [String;6] = ["EDTY".to_string(), "EDDH".to_string(), "KJFK".to_string(), "KSFO".to_string(), "RJTT".to_string(), "VOBL".to_string()];

    for x in &icao_list { 
        /* Determine metar for station */
        let metar = libs::metar::read_metar_text(&x).await;

        if metar == "no metar available".to_string(){
            println!("{}", metar);
            continue;
        }
        println!{"METAR for static list: {}", metar};
    }
}

fn debug_print(input: airport::Airport){
    
    println!("Debug airport:");
    println!("{}", input.icao);
    println!("{}:{}z", input.utc_hour, input.utc_minute);
    println!("Q{}", input.qnh);
    println!("w{}/{}",input.wind_direction, input.wind_speed);
    println!("CAVOK? {}", input.cavok);
    println!("Confidence? {}", input.confidence);
    println!("temp {}/{}", input.temperature, input.dewpoint);
    println!("vis {}", input.visibility);
    for elem in input.clouds{
        println!("cloud: {} in {}", elem.class, elem.height);
    }

}








