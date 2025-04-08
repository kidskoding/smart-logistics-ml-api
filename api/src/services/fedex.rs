use reqwest::header::{HeaderMap, HeaderValue};

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert("X-locale", HeaderValue::from_static("en_US"));
    headers.insert("Authorization", HeaderValue::from_static("Bearer "));
    headers
}

pub async fn run() -> reqwest::Result<()> {
    let client = reqwest::Client::new();
    let input = r#"{
        "tcnInfo": {
        "value": "N552428361Y555XXX",
        "carrierCode": "FDXE",
        "shipDateBegin": "2019-02-13",
        "shipDateEnd": "2019-02-13"
        },
        "includeDetailedScans": true
    }"#;

    let res = client.post("https://apis-sandbox.fedex.com/track/v1/tcn")
        .body(input)
        .headers(construct_headers())
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