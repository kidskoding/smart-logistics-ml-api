use api::services::fedex;

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    /* let _db = db::connect()
        .await
        .expect("could not connect to database!"); */

    fedex::track_multiple_piece_shipment()
        .await
        .expect("could not track shipment!");
    
    Ok(())
}
