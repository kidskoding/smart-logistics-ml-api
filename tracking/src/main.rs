use sqlx::PgPool;
use tracking::services::fedex;

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    let db: PgPool = db::connection::connect()
        .await
        .expect("could not connect to database!");

    let tracking_numbers = vec![
        "128667043726",
        "509466710003247",
        "449044304137821",
        "149331877648230",
        "020207021381215",
        "403934084723025",
        "920241085725456",
        "568838414941",
        "039813852990618",
        "231300687629630",
        "797806677146",
        "377101283611590",
        "852426136339213",
        "797615467620",
        "957794015041323",
        "076288115212522",
        "581190049992",
        "122816215025810",
        "843119172384577",
        "070358180009382"
    ];

    batch_track_and_insert(&tracking_numbers, &db).await?;

    Ok(())
}

async fn batch_track_and_insert(tracking_numbers: &[&str], db: &PgPool) -> sqlx::Result<()> {
    let endpoint = "https://apis-sandbox.fedex.com/track/v1/trackingnumbers";

    let tracking_info_objects: Vec<_> = tracking_numbers.iter().map(|tracking_number|
        {
            serde_json::json!({
                "trackingInfo": [
                    {
                        "trackingNumberInfo": {
                            "trackingNumber": tracking_number
                        },
                    }
                ],
                "includeDetailedScans": true
            })
        }
    ).collect();

    for tracking_info_json in tracking_info_objects {
        let json_input = serde_json::to_string(&tracking_info_json).unwrap();

        let api_call = fedex::track_shipment(&json_input, endpoint)
            .await
            .expect("could not track shipment!");

        let tracking_info = fedex::parse_fedex_response(&api_call.1)
            .expect("could not parse fedex response!");

        db::schema::insert_tracking_info(db, &tracking_info)
            .await
            .unwrap_or_default();

        db::schema::insert_timestamps(db, &tracking_info)
            .await
            .unwrap_or_default();
    }

    Ok(())
}
