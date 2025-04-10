extern crate reqwest;
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};

async fn construct_headers() -> HeaderMap {
    let token = get_fedex_token().await.unwrap();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert("X-locale", HeaderValue::from_static("en_US"));
    headers.insert("Authorization", HeaderValue::from_str(
        &format!("Bearer {}", token)
    ).unwrap());
    
    headers
}

pub async fn get_fedex_token() -> reqwest::Result<String> {
    dotenv().ok();

    let client_id = std::env::var("FEDEX_API_KEY")
        .expect("FEDEX_API_KEY not set");
    let client_secret = std::env::var("FEDEX_SECRET_API_KEY")
        .expect("FEDEX_SECRET_API_KEY not set");

    let input = format!(
        "grant_type=client_credentials&client_id={}&client_secret={}", 
        client_id, 
        client_secret
    );

    let client = reqwest::Client::new(); 
    let res = client
        .post("https://apis-sandbox.fedex.com/oauth/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(input)
        .send()
        .await?;

    let json: serde_json::Value = res.json().await?;
    let access_token: String = json["access_token"].as_str()
        .expect("Failed to get access token")
        .to_string();

    Ok(access_token)
}

pub async fn track_multiple_piece_shipment() -> reqwest::Result<()> {
    let input = r#"{
        "includeDetailedScans": true,
        "associatedType": "STANDARD_MPS",
        "masterTrackingNumberInfo": {
            "shipDateEnd": "2025-04-07",
            "shipDateBegin": "2025-04-05",
            "trackingNumberInfo": {
                "trackingNumberUniqueId": "245822~123456789012~FDEG",
                "carrierCode": "FDXE",
                "trackingNumber": "858488600850"
            }
        },
        "pagingDetails": {
            "resultsPerPage": 56,
            "pagingToken": "38903279038"
        }
    }"#;
    
    let client = reqwest::Client::new();
    
    let res = client.post("https://apis-sandbox.fedex.com/track/v1/associatedshipments")
        .body(input.to_string())
        .headers(construct_headers().await)
        .send()
        .await?;
        
    let status = res.status();
    let headers = res.headers().clone();
    
    let body = res.text().await?;
    
    println!("Status: {}", status);
    println!("Headers:\n{:#?}", headers);
    println!("Body:\n{}", body);

    Ok(())
}