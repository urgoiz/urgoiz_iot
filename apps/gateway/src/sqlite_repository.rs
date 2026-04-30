use crate::domain::{SensorData, SensorError, SensorRepository, SensorType};
use async_trait::async_trait;
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, Pool, Sqlite};
use std::str::FromStr;
use std::time::Duration;

pub struct SqliteRepository {
    pool: Pool<Sqlite>,
}  

impl SqliteRepository {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn std::error::Error>> {

        let pool = Self::setup_pool(database_url).await?;

        let repo = SqliteRepository { pool };

        repo.run_migrations().await?;

        Ok(repo)
    }

    async fn setup_pool(url: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
         let options = SqliteConnectOptions::from_str(url)?
            .create_if_missing(true)
            .busy_timeout(Duration::from_secs(5));

        SqlitePoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect_with(options)
            .await
    }

    async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS sensor_types (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE
            );
            CREATE TABLE IF NOT EXISTS sensors (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                hardware_id TEXT NOT NULL UNIQUE,
                sensor_type_id INTEGER NOT NULL,
                FOREIGN KEY(sensor_type_id) REFERENCES sensor_types(id)
            );
            CREATE TABLE IF NOT EXISTS readings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                sensor_id INTEGER NOT NULL,
                value REAL NOT NULL,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(sensor_id) REFERENCES sensors(id)
            );"
        )
        .execute(&self.pool)
        .await?;
    
        for t in &[
            SensorType::Temperature,
            SensorType::Humidity,
            SensorType::Pressure,
            SensorType::Unknown,
        ] {
            sqlx::query("INSERT OR IGNORE INTO sensor_types (name) VALUES (?1)")
                .bind(t.to_string())
                .execute(&self.pool)
                .await?;
        }

        Ok(())
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
        
        sqlx::query("INSERT OR IGNORE INTO sensors (hardware_id, sensor_type_id) VALUES (?1, ?2)")
            .bind(data.sensor_id.as_str())
            .bind(type_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| SensorError::DatabaseError(e.to_string()))?;
        
        let internal_sensor_id: i64 = sqlx::query_scalar("SELECT id FROM sensors WHERE hardware_id = ?1")
            .bind(data.sensor_id.as_str())
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| SensorError::DatabaseError(e.to_string()))?;

        sqlx::query("INSERT INTO readings (sensor_id, value) VALUES (?1, ?2)")
            .bind(internal_sensor_id)
            .bind(data.value)
            .execute(&mut *tx)
            .await
            .map_err(|e| SensorError::DatabaseError(e.to_string()))?;

        tx.commit().await
            .map_err(|e| SensorError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{SensorData, SensorId, SensorType};

    async fn setup_test_repo() -> SqliteRepository {
        SqliteRepository::new("sqlite::memory:").await.unwrap()
    }

    #[tokio::test]
    async fn test_save_reading_full_flow() {
        let repo = setup_test_repo().await;
        let data = SensorData {
            sensor_id: SensorId::new("test_sensor_01"),
            sensor_type: SensorType::Temperature,
            value: 25.5,
        };
        let result = repo.save_reading(data).await;
        assert!(result.is_ok());

        let sensor_exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM sensors WHERE hardware_id = ?1)")
            .bind("test_sensor_01")
            .fetch_one(&repo.pool)
            .await
            .unwrap();

        assert!(sensor_exists);
    }

    #[tokio::test]
    async fn test_foreign_fey_constraint() {
        let repo = setup_test_repo().await;
        let mut tx = repo.pool.begin().await.unwrap();
        let result = sqlx::query("INSERT INTO readings (sensor_id, value) VALUES (?1, ?2)")
            .bind(9999)
            .bind(10.5)
            .execute(&mut *tx)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_idempotent_sensor_registration() {
        let repo = setup_test_repo().await;
        let sensor_id = "duplicate_sensor";

        let data1 = SensorData {
            sensor_id: SensorId::new(sensor_id),
            sensor_type: SensorType::Temperature,
            value: 10.0,
        };

        let data2 = SensorData {
            sensor_id: SensorId::new(sensor_id),
            sensor_type: SensorType::Temperature,
            value: 20.0,
        };

        repo.save_reading(data1).await.unwrap();
        repo.save_reading(data2).await.unwrap();

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sensors WHERE hardware_id = ?1")
            .bind(sensor_id)
            .fetch_one(&repo.pool)
            .await
            .unwrap();

        assert_eq!(count, 1);
    }
}