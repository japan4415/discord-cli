use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct AutoModerationRule {
    pub id: String,
    pub guild_id: String,
    pub name: String,
    #[tabled(display_with = "display_option")]
    pub event_type: Option<u8>,
    #[tabled(display_with = "display_option")]
    pub trigger_type: Option<u8>,
    #[tabled(skip)]
    pub actions: Option<serde_json::Value>,
    #[tabled(display_with = "display_option")]
    pub enabled: Option<bool>,
}
