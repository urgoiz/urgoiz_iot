use crate::domain::{SensorData, SensorError, SensorRepository};
use async_trait::async_trait;
use sqlx::{SqlitePool, Pool, Sqlite};

pub struct SqliteRepository {
    pool: Pool<Sqlite>,
}  

impl SqliteRepository {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let pool = SqlitePool::connect(database_url).await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS readings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                sensor_id TEXT NOT NULL,
                sensor_type TEXT NOT NULL,
                value REAL NOT NULL,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            )"
        )
        .execute(&pool)
        .await?;

        Ok(SqliteRepository { pool })
    }
}

#[async_trait]
impl SensorRepository for SqliteRepository {
    async fn save_reading(&self, data: SensorData) -> Result<(), SensorError> {
        sqlx::query(
            "INSERT INTO readings (sensor_id, sensor_type, value) VALUES (?1, ?2, ?3)"
        )
        .bind(&data.sensor_id)
        .bind(data.sensor_type.to_string())
        .bind(data.value)
        .execute(&self.pool)
        .await
        .map_err(|e| SensorError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}