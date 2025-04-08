use api::{db, services::{dhl, fedex}};

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    let _db = db::connect()
        .await
        .expect("could not connect to database!");

    dhl::get_tracking_info()
        .await
        .expect("could not get tracking info!");
    
    Ok(())
}
