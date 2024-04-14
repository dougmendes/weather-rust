use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WeatherResponse{
    pub current_weather: CurrentWeather,
}
#[derive(Deserialize, Debug)]
pub struct CurrentWeather{
    temperature: f64,
    windspeed: f64,
    weathercode: i32,
}
impl CurrentWeather{
    pub fn get_temperature(&self) -> &f64{
        &self.temperature
    }

    pub fn get_windspeed(&self) -> &f64{
        &self.windspeed
    }
    pub fn get_weathercode(&self) -> &i32{
        &self.weathercode
    }
}

pub struct Location {
    longitude: String,
    latitude: String
}

impl Location {

    pub fn new(longitude: String, latitude: String) -> Self {
        Self { longitude, latitude }
    }
    pub fn get_longitude(&self) -> &String{
        &self.longitude
    }

    pub fn get_latitude(&self) -> &String{
        &self.latitude
    }
}
