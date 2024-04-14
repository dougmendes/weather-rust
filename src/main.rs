mod application;
mod infrastructure;
mod domain;

use ipgeolocate::{Locator, GeoError};
use tokio;
use reqwest;

use crate::application::service::{get_weather, get_location};
use crate::infrastructure::location_service::{LocationService, RealLocationService};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let ip = reqwest::get("https://api.ipify.org")
        .await?
        .text()
        .await?;

    let service = RealLocationService;
    let location = get_location(&service,&ip).await.unwrap();

    let response = get_weather(&location).await.unwrap();

    println!("Current temperature is {}°C", response.current_weather.get_temperature());
    println!("Current windspeed is {} km/h", response.current_weather.get_windspeed());
    println!("Weather code is {}", response.current_weather.get_weathercode());

    Ok(())
}



mod tests {
    use super::*;
    use mockall::*;
    use async_trait::async_trait;
    use ipgeolocate::Locator as IpGeolocateLocator;

    mock! {
        MockLocationService{}

        #[async_trait]
        impl LocationService for MockLocationService {
            async fn get_location(&self, ip: &str) -> Result<Locator, GeoError>;
        }
    }
    #[tokio::test]
    async fn test_get_location() {
        let mut mock_service = MockMockLocationService::new();


        let mock_locator = IpGeolocateLocator {
            ip: "1.1.1.1".to_string(),
            latitude: "1.2.3.5.6".to_string(),
            longitude:"1.2.3.5.6".to_string(),
            city: "Localhost City".to_string(),
            region: "Localhost Region".to_string(),
            country: "Nowhere Land".to_string(),
            timezone: "GMT".to_string(),
            isp: "ISP_TR123".to_string()
        };
        

        mock_service.expect_get_location()
                    .with(predicate::eq("1.1.1.1"))
                    .times(1)
                    .return_once(| _ | Ok(mock_locator));
        
        let result = get_location(&mock_service, "1.1.1.1").await;
        assert!(result.is_ok());
    }
}