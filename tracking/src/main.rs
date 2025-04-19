use api::{db, services::fedex};

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    let _db = db::connect()
        .await
        .expect("could not connect to database!");

    let api_call = fedex::track_shipment(r#"
        {
            "includeDetailedScans": true,
            "trackingInfo": [
                {
                    "shipDateBegin": "2020-03-29",
                    "shipDateEnd": "2020-04-01",
                    "trackingNumberInfo": {
                        "trackingNumber": "128667043726",
                        "carrierCode": "FDXE",
                        "trackingNumberUniqueId": "245822~123456789012~FDEG"
                    }
                }
            ]
        }"#, 
        "https://apis-sandbox.fedex.com/track/v1/trackingnumbers"
    ).await.expect("could not track shipment!");

    let tracking_info = fedex::parse_fedex_response(&api_call.1)
        .expect("could not parse fedex response!");
    print!("{}", tracking_info);

    Ok(())
}
