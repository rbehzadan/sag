use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    #[serde(default)]
    pub required: bool,

    #[serde(default)]
    pub roles: Vec<String>,

    #[serde(default)]
    pub auth_type: AuthType,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum AuthType {
    #[default]
    Bearer,
    Basic,
    ApiKey,
}
