use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct GatewayInfo {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct GatewayBotInfo {
    pub url: String,
    #[tabled(display = "display_option")]
    pub shards: Option<u64>,
    #[tabled(skip)]
    pub session_start_limit: Option<serde_json::Value>,
}
