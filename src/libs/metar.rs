use serde::Deserialize; 
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
    struct MetarText {
        results: i32,
        data: [String; 1],
    }

pub async fn read_metar_text(input: &String) -> String {

    

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

    let object: MetarText = serde_json::from_str(&res).unwrap();

    let metar = object.data[0].to_string();

    return metar;
}