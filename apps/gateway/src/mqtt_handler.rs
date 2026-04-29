use crate::domain::{SensorData, SensorError, SensorRepository};
use async_trait::async_trait;

pub struct MqttHandler<R: SensorRepository> {
    repository: R,
}

impl<R: SensorRepository> MqttHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle_message(
        &self,
        payload: &[u8],
        parser_fn: fn(&[u8]) -> Result<SensorData, SensorError>,
    ) -> Result<(), String> {
        let data = parser_fn(payload)
            .map_err(|e| format!("Failed to parse sensor data | Reason: {:?}", e))?;

        self.repository
            .save_reading(data)
            .await
            .map_err(|e| format!("Database error | Reason: {:?}", e))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::SensorType;
    use std::sync::{Arc, Mutex};

    fn mock_success_parser(_payload: &[u8]) -> Result<SensorData, SensorError> {
        Ok(SensorData {
            sensor_type: SensorType::Temperature,
            value: 22.5,
        })
    }

    fn mock_fail_parser(_payload: &[u8]) -> Result<SensorData, SensorError> {
        Err(SensorError::InvalidPayload("Cannot parse".to_string()))
    }

    #[tokio::test]
    async fn test_handle_valid_message_with_injected_parser() {

        let repo = MockRepository {
            saved_data: Arc::new(Mutex::new(vec![])),
        };
        let handler = MqttHandler::new(repo);

        let result = handler.handle_message(b"any_payload", mock_success_parser).await;

        assert!(result.is_ok());
    }   

    #[tokio::test]
    async fn test_handle_invalid_message_with_injected_parser() {
        let repo = MockRepository {
            saved_data: Arc::new(Mutex::new(vec![])),
        };
        let handler = MqttHandler::new(repo);

        let result = handler.handle_message(b"error_reading", mock_fail_parser).await;
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to parse sensor data"));
    }

    struct MockRepository {
        saved_data: Arc<Mutex<Vec<SensorData>>>,
    }

    #[async_trait]
    impl SensorRepository for MockRepository {
        async fn save_reading(&self, data: SensorData) -> Result<(), SensorError> {
            self.saved_data.lock().unwrap().push(data.clone());
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_handler_calls_repository() {
        let saved_data = Arc::new(Mutex::new(vec![]));
        let repo = MockRepository {
            saved_data: saved_data.clone(),
        };
        let handler = MqttHandler::new(repo);

        let result = handler.handle_message(b"some_payload", mock_success_parser).await;

        assert!(result.is_ok());
        let data_in_repo = saved_data.lock().unwrap();
        assert_eq!(data_in_repo.len(), 1);
        assert_eq!(data_in_repo[0].sensor_type, SensorType::Temperature);
        assert_eq!(data_in_repo[0].value, 22.5);
    }
}