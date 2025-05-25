use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    #[serde(default)]
    pub required: bool,

    #[serde(default)]
    pub roles: Vec<String>,
}
