use crate::domain::models::{Location, WeatherResponse};
use crate::infrastructure::location_service::LocationService;
use ipgeolocate::GeoError;
use reqwest;

pub async fn get_location<S: LocationService>(service: &S, ip: &str)-> Result<Location, GeoError>{
    match service.get_location(ip).await{
        Ok(response) => {
            let location = Location::new(response.longitude, response.latitude);
            println!("Consulting weather for{:?}", response.city);
            Ok(location)
        }
        Err(error) => {
            println!("Error: {}", error);
            Err(error)
        }
    }
}

pub async fn get_weather(location: &Location) -> Result<WeatherResponse, reqwest::Error>{
    let weather_url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
                            location.get_latitude(), 
                            location.get_longitude());
    let response = reqwest::get(weather_url).await?.json::<WeatherResponse>().await?;
    Ok(response)
}