use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct Thread {
    pub id: String,
    #[tabled(display = "display_option")]
    pub guild_id: Option<String>,
    #[tabled(display = "display_option")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub channel_type: u8,
    #[tabled(display = "display_option")]
    pub parent_id: Option<String>,
    #[tabled(display = "display_option")]
    pub owner_id: Option<String>,
    #[tabled(display = "display_option")]
    pub message_count: Option<u64>,
    #[tabled(display = "display_option")]
    pub member_count: Option<u64>,
    #[tabled(display = "display_option")]
    pub archived: Option<bool>,
    #[tabled(display = "display_option")]
    pub locked: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct ThreadMember {
    #[tabled(display = "display_option")]
    pub id: Option<String>,
    #[tabled(display = "display_option")]
    pub user_id: Option<String>,
    pub join_timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveThreadsResponse {
    pub threads: Vec<Thread>,
    pub members: Vec<ThreadMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchivedThreadsResponse {
    pub threads: Vec<Thread>,
    pub members: Vec<ThreadMember>,
    pub has_more: bool,
}
