#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ControlApiConfig {
    pub enabled: bool,
    pub bind_address: String,
    pub api_key: String,
}
