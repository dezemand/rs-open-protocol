#[derive(Debug, Default)]
pub struct Config {
    mqtt_type: MqttType,
    mqtt_connection: MqttConnection,
    connections: Vec<OpenProtocolConnections>
}

#[derive(Debug, Default)]
pub struct MqttConnection {
    pub host: String,
    pub port: u16,
    pub auth: MqttAuth,
    pub client_id: Option<String>,
    pub clean_session: Option<bool>,
    // Secure SSL connection to the server
    // pub secure
}

#[derive(Debug, Default)]
pub struct OpenProtocolConnections {
    name: String
}

#[derive(Debug)]
pub enum MqttType {
    Flat { root: String },
    #[cfg(feature = "sparkplug")]
    SparkplugB { group_id: String, eon_id: String }
}

#[derive(Debug)]
pub enum MqttAuth {
    None,
    Basic { username: String, password: String },
    Certificate {},
    CertificateAndBasic { username: String, password: String }
}

impl Default for MqttType {
    fn default() -> Self {
        Self::Flat { root: String::default() }
    }
}

impl Default for MqttAuth {
    fn default() -> Self {
        Self::None
    }
}
