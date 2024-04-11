use ipgeolocate::{Locator, Service};
use tokio;
use reqwest;



#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let ip = reqwest::get("https://api.ipify.org")
        .await?
        .text()
        .await?;
    get_location(&ip).await;

    Ok(())
}

async fn get_location(ip: &str) -> (){
    let service = Service::IpApi;

    match Locator::get(ip, service).await{
        Ok(response) => {
            println!("{:?}", response.city);
        },
        Err(error) => println!("Error: {}", error),
    }
    
}