use serde::Deserialize; 
use serde::Serialize;

use crate::libs::api_key::read_api_key;

#[derive(Serialize, Deserialize, Debug)]
    struct MetarText {
        results: i32,
        data: [String; 1],
    }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetarData {
    pub results: i64,
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    pub icao: String,
    pub barometer: Barometer,
    pub clouds: Vec<Cloud>,
    pub dewpoint: Dewpoint,
    pub elevation: Elevation,
    #[serde(rename = "flight_category")]
    pub flight_category: String,
    pub humidity: Humidity,
    pub observed: String,
    pub station: Station,
    pub temperature: Temperature,
    #[serde(rename = "raw_text")]
    pub raw_text: String,
    pub visibility: Visibility,
    pub wind: Wind,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Barometer {
    pub hg: f64,
    pub hpa: f64,
    pub kpa: f64,
    pub mb: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cloud {
    pub code: String,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dewpoint {
    pub celsius: i64,
    pub fahrenheit: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Elevation {
    pub feet: f64,
    pub meters: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Humidity {
    pub percent: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    pub geometry: Geometry,
    pub location: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geometry {
    pub coordinates: Vec<f64>,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Temperature {
    pub celsius: i64,
    pub fahrenheit: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Visibility {
    pub miles: String,
    #[serde(rename = "miles_float")]
    pub miles_float: f64,
    pub meters: String,
    #[serde(rename = "meters_float")]
    pub meters_float: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wind {
    pub degrees: u16,
    #[serde(rename = "speed_kph")]
    pub speed_kph: u16,
    #[serde(rename = "speed_kts")]
    pub speed_kts: u16,
    #[serde(rename = "speed_mph")]
    pub speed_mph: u16,
    #[serde(rename = "speed_mps")]
    pub speed_mps: u16,
}

pub async fn get_wind(icao: &String) -> String {
    let object: MetarData= read_metar_data(&icao).await;

    let wind = object.data[0].wind.speed_kph.to_string();

    return wind
}
    

pub async fn read_metar_text(icao: &String) -> String {

    let api_key = read_api_key();
    let url = create_metar_text_url(&icao, &api_key);

    // Perform the HTTP request
    let response =  reqwest::get(url)
        .await
        .unwrap()
        .text()
        .await;  

     //let res = response.unwrap();

     let res = match response {
        Ok(v) => v,
        Err(_err) => return "no metar found".to_string(),
     };

    // Check if results is not 0
    if res.contains(":0"){
        let blank_metar = "no metar available".to_string();
        return blank_metar

    }
    
    // Convert into JSON object 

    // TODO: results = 0 -> No unwrapping into MetarText allowed because no data is available

    let object: MetarText = serde_json::from_str(&res).unwrap();

    let metar = object.data[0].to_string();

    return metar;
}

pub async fn read_metar_data(icao: &String) -> MetarData {

    let api_key = read_api_key();
    let url = create_metar_data_url(&icao, &api_key);

    
    // Perform the HTTP request
    let response =  reqwest::get(url)
        .await
        .unwrap()
        .text()
        .await;  

     //let res = response.unwrap();

     let res = match response {
        Ok(v) => v,
        Err(_err) => "no metar data found".to_string(),
     };

    // Convert into JSON object 

    let object: MetarData = serde_json::from_str(&res).unwrap();

    return object;
}

fn create_metar_text_url(icao: &String, api_key: &String) -> String {
    
    let url: String = "https://api.checkwx.com/metar/".to_string() + &icao + "?x-api-key=" + &api_key;

    return url;
}

fn create_metar_data_url(icao: &String, api_key: &String) -> String {
    
    let url: String = "https://api.checkwx.com/metar/".to_string() + &icao + "/decoded?x-api-key=" + &api_key;

    return url;
}

pub async fn update(icao: &String) -> String {
    let object: MetarData= read_metar_data(&icao).await;

    let wind = object.data[0].wind.speed_kph.to_string();

    return wind
}