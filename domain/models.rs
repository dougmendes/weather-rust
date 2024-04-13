#[derive(Deserialize, Debug)]
pub struct WeatherResponse{
    current_weather: CurrentWeather,
}
#[derive(Deserialize, Debug)]
pub struct CurrentWeather{
    temperature: f64,
    windspeed: f64,
    weathercode: i32,
}

pub struct Location {
    longitude: String,
    latitude: String
}