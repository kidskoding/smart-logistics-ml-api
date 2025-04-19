use std::env;
use chrono::DateTime;
use db::tracking_info::TrackingInfo;
use dotenv::dotenv;
use reqwest::{header::{HeaderMap, HeaderValue}, StatusCode};
use serde_json::Value;

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
    let json_value: Result<Value, serde_json::Error> = serde_json::from_str(&body);

    if let Ok(value) = json_value {
        let pretty_body = serde_json::to_string_pretty(&value).unwrap();
        Ok((status, pretty_body))
    } else {
        eprintln!("could not pretty format json response. outputting raw response");
        Ok((status, body))
    }
}

pub fn parse_fedex_response(response_json: &str) -> serde_json::Result<TrackingInfo> {
    let json: Value = serde_json::from_str(response_json)?;
    let track_results = &json["output"]["completeTrackResults"][0]["trackResults"][0];

    let tracking_info = TrackingInfo {
        tracking_id: track_results["trackingNumberInfo"]["trackingNumber"]
            .as_str()
            .expect("missing tracking number!")
            .parse::<i64>()
            .expect("failed to parse tracking number as i64"),

        carrier: track_results["trackingNumberInfo"]["carrierCode"]
            .as_str()
            .expect("missing carrier code!")
            .to_string(),

        delivery_date: {
            let delivery_date_str = track_results["dateAndTimes"]
                .as_array()
                .and_then(|dates| {
                    dates.iter().find(|date| {
                        date["type"].as_str().unwrap_or("") == "ACTUAL_DELIVERY"
                    })
                })
                .and_then(|date| date["dateTime"].as_str())
                .expect("missing delivery date!")
                .to_string();

            DateTime::parse_from_rfc3339(&delivery_date_str)
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .expect("failed to parse delivery date")
                .to_string()
        },

        status: track_results["latestStatusDetail"]["statusByLocale"]
            .as_str()
            .expect("missing status")
            .to_string(),

        location: {
            let location_data = &track_results["latestStatusDetail"]["scanLocation"];
            let city = location_data["city"]
                .as_str()
                .expect("missing city")
                .to_string();
            let state = location_data["stateOrProvinceCode"]
                .as_str()
                .expect("missing state")
                .to_string();

            (city, state)
        },

        timestamps: track_results["scanEvents"]
            .as_array()
            .map(|events| {
                events.iter().filter_map(|event| {
                    let date = event["date"].as_str()?;
                    let description = event["eventDescription"].as_str()?;
                    Some((description.to_string(), date.to_string()))
                }).collect::<Vec<_>>()
            })
            .unwrap_or_default(),

        dimensions: {
            let dimensions_data = &track_results["packageDetails"]["weightAndDimensions"]["dimensions"][0];
            [
                dimensions_data["length"].as_i64().unwrap_or(0),
                dimensions_data["width"].as_i64().unwrap_or(0),
                dimensions_data["height"].as_i64().unwrap_or(0),
            ]
        },

        weight: track_results["packageDetails"]["weightAndDimensions"]["weight"][0]["value"]
            .as_str()
            .and_then(|w| w.parse::<f64>().ok())
            .unwrap_or(0.0),
    };

    Ok(tracking_info)
}