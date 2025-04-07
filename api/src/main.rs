use api::db::connect;

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    let db = connect()
        .await
        .expect("could not connect to database!");
    
    Ok(())
}
