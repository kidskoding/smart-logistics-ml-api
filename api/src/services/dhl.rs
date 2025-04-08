extern crate reqwest;

use std::env;
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};

pub async fn get_tracking_info() -> reqwest::Result<()> {
    dotenv().ok();
    let dhl_api_key = env::var("DHL_API_KEY")
        .expect("DHL_API_KEY not set");

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "DHL-API-Key",
        HeaderValue::from_str(&dhl_api_key)
                .expect("failed to parse DHL_API_KEY!")
    );

    let request = client
        .get( "https://api-test.dhl.com/track/shipments?trackingNumber=00340434161094030000")
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}