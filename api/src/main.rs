use api::services::fedex;

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    /* let _db = db::connect()
        .await
        .expect("could not connect to database!"); */

    fedex::track_shipment(r#"
        {
            "tcnInfo": {
                "value": "N552428361Y555XXX",
                "carrierCode": "FDXE",
                "shipDateBegin": "2025-04-07",
                "shipDateEnd": "2025-04-09"
            },
            "includeDetailedScans": true
        }"#
    , "https://apis-sandbox.fedex.com/track/v1/tcn")
        .await
        .expect("could not track shipment!");
    
    Ok(())
}
