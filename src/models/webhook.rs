use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct Webhook {
    pub id: String,
    #[serde(rename = "type")]
    pub webhook_type: u8,
    #[tabled(display_with = "display_option")]
    pub guild_id: Option<String>,
    #[tabled(display_with = "display_option")]
    pub channel_id: Option<String>,
    #[tabled(display_with = "display_option")]
    pub name: Option<String>,
    #[tabled(display_with = "display_option")]
    pub avatar: Option<String>,
    #[tabled(display_with = "display_option")]
    pub token: Option<String>,
    #[tabled(display_with = "display_option")]
    pub url: Option<String>,
}
