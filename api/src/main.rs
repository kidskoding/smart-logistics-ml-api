use api::services::fedex;

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    /* let _db = db::connect()
        .await
        .expect("could not connect to database!"); */

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
    println!("Status: {}", api_call.0);
    println!("Body: \n{}", api_call.1);
    
    Ok(())
}
