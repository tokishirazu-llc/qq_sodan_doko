use std::env;

use futures::executor::block_on;
use serde::Deserialize;

use crate::model::area::Area;

#[derive(Deserialize, Debug)]
struct GoogleMapResponse {
    results: Vec<GoogleMapResult>,
}
#[derive(Deserialize, Debug)]
struct GoogleMapResult {
    address_components: Vec<GoogleMapAddressComponent>,
}
#[derive(Deserialize, Debug)]
struct GoogleMapAddressComponent {
    long_name: String,
    short_name: String,
    types: Vec<String>,
}

fn get_map_endpoint(lat: f32, lng: f32) -> String {
    format!(
        "https://maps.googleapis.com/maps/api/geocode/json?language=ja&latlng={},{}&key={}",
        lat,
        lng,
        env::var("GOOGLE_API_KEY").unwrap()
    )
}

pub fn get_address_from_latlng(lat: f32, lng: f32) -> Result<Area, reqwest::Error> {
    let response: GoogleMapResponse = block_on(
        block_on(
            reqwest::Client::new()
                .get(&get_map_endpoint(lat, lng))
                .send(),
        )?
        .json(),
    )?;

    let mut area = Area {
        pref: "".to_string(),
        city: "".to_string(),
    };
    response
        .results
        .into_iter()
        .flat_map(|r| r.address_components.into_iter())
        .for_each(|address_component| {
            let types = address_component.types;
            if types.contains(&"administrative_area_level_1".to_string()) {
                area.pref = address_component.long_name;
            } else if types.contains(&"locality".to_string()) && !types.contains(&"colloquial_area".to_string()) {
                area.city = address_component.long_name;
            }
        });

    Ok(area)
}
