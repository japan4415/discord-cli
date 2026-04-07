use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct GuildScheduledEvent {
    pub id: String,
    pub guild_id: String,
    pub name: String,
    #[tabled(display_with = "display_option")]
    pub description: Option<String>,
    #[tabled(display_with = "display_option")]
    pub scheduled_start_time: Option<String>,
    #[tabled(display_with = "display_option")]
    pub scheduled_end_time: Option<String>,
    #[tabled(display_with = "display_option")]
    pub entity_type: Option<u8>,
    #[tabled(display_with = "display_option")]
    pub status: Option<u8>,
    #[tabled(display_with = "display_option")]
    pub creator_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct EventUser {
    pub user_id: String,
    #[tabled(skip)]
    pub member: Option<serde_json::Value>,
}
