use crate::domain::{SensorData, SensorError, SensorRepository, SensorType};
use async_trait::async_trait;
use sqlx::{SqlitePool, Pool, Sqlite};

pub struct SqliteRepository {
    pool: Pool<Sqlite>,
}  

impl SqliteRepository {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let pool = SqlitePool::connect(database_url).await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS sensor_types (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE
            );
            CREATE TABLE IF NOT EXISTS sensors (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                hardware_id TEXT NOT NULL UNIQUE,
                sensor_type TEXT NOT NULL,
                FOREIGN KEY(sensor_type) REFERENCES sensor_types(id)
            );
            CREATE TABLE IF NOT EXISTS readings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                sensor_id INTEGER NOT NULL,
                value REAL NOT NULL,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(sensor_id) REFERENCES sensors(id)
            );"
        )
        .execute(&pool)
        .await?;
    
        for t in &[SensorType::Temperature, SensorType::Humidity, SensorType::Pressure, SensorType::Unknown] {
            sqlx::query("INSERT OR IGNORE INTO sensor_types (name) VALUES (?1)")
                .bind(t.to_string())
                .execute(&pool)
                .await?;
        }

        Ok(SqliteRepository { pool })
    }
}

#[async_trait]
impl SensorRepository for SqliteRepository {
    async fn save_reading(&self, data: SensorData) -> Result<(), SensorError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| SensorError::DatabaseError(e.to_string()))?;

        let type_id: i64 = sqlx::query_scalar("SELECT id FROM sensor_types WHERE name = ?1")
            .bind(data.sensor_type.to_string())
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| SensorError::DatabaseError(format!("Type not supported: {}", e)))?;
        
        sqlx::query("INSERT OR IGNORE INTO sensors (id, type_id) VALUES (?1, ?2)")
            .bind(&data.sensor_id.as_str())
            .bind(type_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| SensorError::DatabaseError(e.to_string()))?;
        
        sqlx::query("INSERT INTO readings (sensor_id, value) VALUES (?1, ?2)"
        )
        .bind(&data.sensor_id.as_str())
        .bind(data.value)
        .execute(&mut *tx)
        .await
        .map_err(|e| SensorError::DatabaseError(e.to_string()))?;

        tx.commit().await
            .map_err(|e| SensorError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}