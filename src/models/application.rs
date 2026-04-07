use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct ApplicationCommand {
    pub id: String,
    pub application_id: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    #[tabled(display = "display_option")]
    pub command_type: Option<u8>,
    #[tabled(display = "display_option")]
    pub guild_id: Option<String>,
    #[tabled(skip)]
    pub options: Option<Vec<serde_json::Value>>,
}
