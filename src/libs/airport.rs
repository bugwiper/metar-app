use crate::libs::metar;

pub struct Cloud {
    pub class: String,
    pub base: u16,
}

pub struct Airport {
    pub icao: String,
    pub name: String,
    pub metar: String,
    pub utc_hour: u8,
    pub utc_minute: u8,
    pub day: u8,
    pub wind_direction: u16,
    pub wind_speed: u16,
    pub visibility: u16,
    pub qnh: u16,
    pub temperature: i8,
    pub dewpoint: i8,
    pub cavok: bool,
    pub confidence: bool,
    pub clouds: Vec<Cloud>,
}

impl Airport {
    
    pub fn new(airport: &String) -> Airport{
        
        Airport{
            icao: airport.to_string(),
            name: "default".to_string(),
            metar: "n/a".to_string(),
            utc_hour: 0,
            utc_minute: 0,
            day: 0,
            wind_direction: 0,
            wind_speed: 0,
            visibility: 0,
            qnh: 0,
            temperature: 0,
            dewpoint: 0,
            cavok: false,
            confidence: false,
            clouds: Vec::new(),
        }
    }
}

pub async fn update(airport:Airport) -> Airport{

    let mut update = Airport::new(&airport.icao);
    //TODO: Incorporate wind VARIABLE
    let object: metar::MetarData = metar::read_metar_data(&airport.icao).await;
    update.metar = object.data[0].raw_text.to_string();
    update.utc_hour = 12;
    update.utc_minute = 20;
    update.wind_direction = object.data[0].wind.degrees;
    update.wind_speed = object.data[0].wind.speed_kts;
    update.visibility = object.data[0].visibility.meters_float as u16;
    // TODO: Visibility not working yet
    update.qnh = object.data[0].barometer.hpa as u16;
    update.temperature = object.data[0].temperature.celsius as i8;
    update.dewpoint = object.data[0].dewpoint.celsius as i8;
    update.cavok = false; 
    // TODO: Incorporate CAVOK
    update.confidence = true;
    update.clouds = get_cloud_vector(&object.data[0].clouds);

    return update;

}

fn get_cloud_vector(clouds: &Vec<metar::Cloud>) -> Vec<Cloud> {
    
    let mut new: Vec<Cloud> = Vec::new();
    
    for elem in clouds {
        let my_cloud: Cloud = Cloud {class: elem.code.to_string(), base: elem.feet as u16};
        new.push(my_cloud);
    }
    
    return new
}