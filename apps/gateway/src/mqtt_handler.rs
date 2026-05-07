use crate::domain::{SensorData, SensorRepository};
use crate::error::GatewayError;

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
        parser_fn: fn(&[u8]) -> Result<SensorData, GatewayError>,
    ) -> Result<(), GatewayError> {
        let data = parser_fn(payload)
            .map_err(|e| GatewayError::InvalidData(format!("{:?}", e)))?;

        self.repository
            .save_reading(data)
            .await
            .map_err(|e| GatewayError::DatabaseError(format!("{:?}", e)))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{SensorType, SensorId};
    use std::sync::{Arc, Mutex};
    use async_trait::async_trait;

    fn mock_success_parser(_payload: &[u8]) -> Result<SensorData, GatewayError> {
        Ok(SensorData {
            sensor_id: SensorId::new("mqtt_test".to_string()),
            sensor_type: SensorType::Temperature,
            value: 22.5,
        })
    }

    fn mock_fail_parser(_payload: &[u8]) -> Result<SensorData, GatewayError> {
        Err(GatewayError::InvalidData("Cannot parse".to_string()))
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
        assert!(matches!(result, Err(GatewayError::InvalidData(_))));
    }

    struct MockRepository {
        saved_data: Arc<Mutex<Vec<SensorData>>>,
    }

    #[async_trait]
    impl SensorRepository for MockRepository {
        async fn save_reading(&self, data: SensorData) -> Result<(), GatewayError> {
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