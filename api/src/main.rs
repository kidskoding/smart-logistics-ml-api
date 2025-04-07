use api::{db, services::fedex};

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    /* let db = db::connect()
        .await
        .expect("could not connect to database!"); */

    fedex::run()
        .await
        .expect("could not run FedEx API!");
    
    Ok(())
}
