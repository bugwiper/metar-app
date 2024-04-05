mod libs;
use std::io;


#[tokio::main]
async fn main() {
    
    loop {  

        let icao: String;
        let metar: String;
        let station: String;
        let wind: String;

        /* Get user input for ICAO code */
        println!("Please enter ICAO code for your target airport:");
        println!("(e.g. EDDS for Stuttgart airport)");
        println!("(Type EXIT to exit)");

        
        // Create a mutable String to store the user input
        let mut input = String::new();



        // Read user input from the command line
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let trimmed = &input[..input.len() - 1];
        let new: String = trimmed.to_string();


        /* Validate exit statement */
        if &new == "EXIT" || &new == "exit" {
            break;
        }

        /* Validate user input */
        if !validate_user_input(&new){
            println! ("Invalid ICAO code");
            continue;
        }

        /* Determine ICAO code for target airport */
        icao = new;

        
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
        println!("");
        println!("");

}


}

fn validate_user_input(input: &String) -> bool {

    if input.len() == 4 {
        return true 
    } else {
        return false
    }
}








