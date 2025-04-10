use std::env;
use dotenv::dotenv;
use reqwest::{header::{HeaderMap, HeaderValue}, StatusCode};

pub async fn get_token() -> reqwest::Result<String> {
    dotenv().ok();

    let client_id = env::var("FEDEX_API_KEY")
        .expect("FEDEX_API_KEY not set");
    let client_secret = env::var("FEDEX_SECRET_API_KEY")
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

async fn construct_headers() -> HeaderMap {
    let token = get_token().await.unwrap();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert("X-locale", HeaderValue::from_static("en_US"));
    headers.insert("Authorization", HeaderValue::from_str(
        &format!("Bearer {}", token)
    ).unwrap());
    
    headers
}

pub async fn track_shipment(
    json_input: &str, 
    endpoint: &str
) -> reqwest::Result<(StatusCode, String)> {  
    let client = reqwest::Client::new();
    
    let res = client.post(endpoint)
        .body(json_input.to_string())
        .headers(construct_headers().await)
        .send()
        .await?;
        
    let status = res.status();
    let body = res.text().await?;
    let json_value: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&body);

    match json_value {
        Ok(value) => {
            let pretty_body = serde_json::to_string_pretty(&value).unwrap();
            Ok((status, pretty_body))
        },
        Err(_) => {
            eprintln!("could not pretty format json response. outputting raw response");
            Ok((status, body))
        }
    }
}