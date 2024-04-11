use ipgeolocate::{Locator, Service, GeoError};
use tokio;
use reqwest;
use async_trait::async_trait;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let ip = reqwest::get("https://api.ipify.org")
        .await?
        .text()
        .await?;

    let service = RealLocationService;

    get_location(&service,&ip).await;

    Ok(())
}

async fn get_location<S: LocationService>(service: &S, ip: &str){
    match service.get_location(ip).await{
        Ok(response) => println!("{:?}", response.city),
        Err(error) => println!("Error: {}", error),
    }  
}

#[async_trait]
pub trait LocationService {
    async fn get_location(&self, ip: &str) -> Result<Locator, GeoError>;
}

struct RealLocationService;

#[async_trait]
impl LocationService for RealLocationService {
    async fn get_location(&self, ip: &str) -> Result<Locator, GeoError>{
        Locator::get(ip, Service::IpApi).await
    }
    
}