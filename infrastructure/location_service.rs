use async_trait::async_trait;
use ipgeolocate::{Locator, Service, GeoError};

#[async_trait]
pub trait LocationService {
    async fn get_location(&self, ip: &str) -> Result<Locator, GeoError>;
}

pub struct RealLocationService;

#[async_trait]
impl LocationService for RealLocationService{
    async fn get_location(&self, ip: &str) -> Result<Locator, GeoError>{
        Locator::get(ip, Service::IpApi).await
    }
}