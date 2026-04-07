use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;
use super::user::User;

fn display_author(u: &User) -> String {
    format!("{}({})", u.username, u.id)
}

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct Message {
    pub id: String,
    pub channel_id: String,
    #[tabled(display = "display_author")]
    pub author: User,
    #[tabled(display = "display_option")]
    pub content: Option<String>,
    pub timestamp: String,
    #[tabled(display = "display_option")]
    pub edited_timestamp: Option<String>,
    #[tabled(skip)]
    #[serde(flatten)]
    pub extra: serde_json::Value,
}
