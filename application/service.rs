use crate::interfaces::location_service::LocationService;
use crate::domain::models::{Location, WeatherResponse};
use reqwest;
use async_trait::async_trait;
use serde::Deserialize;


pub async fn get_weather(location: &Location) -> Result<WeatherResponse, reqwest::Error>{
    let weather_url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
                            location.latitude, 
                            location.longitude);
    let response = reqwest::get(weather_url).await?.json::<WeatherResponse>().await?;
    Ok(response)
}