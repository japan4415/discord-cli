use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct Channel {
    pub id: String,
    #[serde(rename = "type")]
    pub channel_type: u8,
    #[tabled(display = "display_option")]
    pub guild_id: Option<String>,
    #[tabled(display = "display_option")]
    pub name: Option<String>,
    #[tabled(display = "display_option")]
    pub topic: Option<String>,
    #[tabled(display = "display_option")]
    pub nsfw: Option<bool>,
    #[tabled(display = "display_option")]
    pub position: Option<i32>,
    #[tabled(skip)]
    #[serde(flatten)]
    pub extra: serde_json::Value,
}
