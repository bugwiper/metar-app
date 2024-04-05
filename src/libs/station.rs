use serde::Deserialize; 
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationData {
    pub results: i64,
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    pub icao: String,
    pub city: String,
    pub country: Country,
    pub elevation: Elevation,
    pub geometry: Geometry,
    pub latitude: Latitude,
    pub longitude: Longitude,
    pub location: String,
    pub name: String,
    pub region: Region,
    pub status: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub code: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Elevation {
    pub feet: f64,
    pub meters: f64,
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
pub struct Latitude {
    pub decimal: f64,
    pub degrees: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Longitude {
    pub decimal: f64,
    pub degrees: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    pub code: String,
    pub name: String,
}



pub async fn read_station_info(input: &String) -> String {

    // Perform the HTTP request
    let response =  reqwest::get(input)
       .await
        .unwrap()
        .text()
       .await;  

     //let res = response.unwrap();

     let res = match response {
        Ok(v) => v,
        Err(_err) => return "no metar found".to_string(),
     };


    // Convert into JSON object 

    let object: StationData = serde_json::from_str(&res).unwrap();

    //let metar = object.data[0].to_string();
    let name = object.data[0].name.to_string() + " " + &object.data[0].city;

    return name;
}