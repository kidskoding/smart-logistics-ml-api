use sqlx::PgPool;
use crate::tracking_info::TrackingInfo;

pub async fn insert_tracking_info(pool: &PgPool, tracking_info: &TrackingInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
            INSERT INTO tracking_info (tracking_id, carrier, delivery_date, status, city, state, length, width, height, weight)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        "#)
        .bind(&tracking_info.tracking_id)
        .bind(&tracking_info.carrier)
        .bind(&tracking_info.delivery_date)
        .bind(&tracking_info.status)
        .bind(&tracking_info.location.0)
        .bind(&tracking_info.location.1)
        .bind(&tracking_info.dimensions[0])
        .bind(&tracking_info.dimensions[1])
        .bind(&tracking_info.dimensions[2])
        .bind(&tracking_info.weight)
        .execute(pool)
        .await?;
    
    Ok(())
}

pub async fn insert_timestamps(pool: &PgPool, tracking_info: &TrackingInfo) -> sqlx::Result<()> {
    for (event, datetime) in &tracking_info.timestamps {
        let (date, time) = datetime.split_once('T').unwrap_or((datetime, ""));
        sqlx::query(
            r#"
                INSERT INTO timestamps (tracking_info_id, event, date, time)
                VALUES ($1, $2, $3, $4)
            "#)
            .bind(&tracking_info.tracking_id)
            .bind(event)
            .bind(date)
            .bind(time)
            .execute(pool)
            .await?;
    }
    
    Ok(())
}