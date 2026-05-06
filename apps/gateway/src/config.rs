use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub mqtt: MqttSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MqttSettings {
    pub host: String,
    pub port: u16,
    pub topic: String,
}


impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = config::Config::builder()
            .set_default("database.url", "sqlite:gateway.db")?
            .set_default("mqtt.host", "localhost")?
            .set_default("mqtt.port", 1883)?
            .set_default("mqtt.topic", "sensors/#")?

            .add_source(config::File::with_name("config/base").required(false))
            .add_source(config::File::with_name(&format!("config/{}", run_mode)).required(false))

            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .build()?;

        s.try_deserialize()
    }

    pub fn new_test_default() -> Self {
        Self {
            database: DatabaseSettings {
                url: "sqlite::memory:".into(),
            },
            mqtt: MqttSettings {
                host: "localhost".into(),
                port: 1883,
                topic: "test/#".into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_from_struct() {
        let settings = Settings::new_test_default();
        assert_eq!(settings.mqtt.port, 1883);
        assert!(settings.database.url.contains("sqlite"));
    }
}