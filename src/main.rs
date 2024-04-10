

use ipgeolocate::{Locator, Service};
use tokio;
use local_ip_address::local_ip;



#[tokio::main]
async fn main() {
    let my_local_ip = local_ip().unwrap().to_string();
    get_location(&my_local_ip).await;
}

async fn get_location(ip: &str) -> (){
    let service = Service::IpApi;

    match Locator::get(ip, service).await{
        Ok(response) => {
            println!("{:?}", response);
        },
        Err(error) => println!("Error: {}", error),
    }
    
}