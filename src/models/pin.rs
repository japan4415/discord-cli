use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct PinnedMessage {
    pub id: String,
    pub channel_id: String,
    #[tabled(display_with = "display_option")]
    pub content: Option<String>,
    #[tabled(skip)]
    pub author: Option<serde_json::Value>,
    #[tabled(display_with = "display_option")]
    pub timestamp: Option<String>,
    #[tabled(display_with = "display_option")]
    pub pinned: Option<bool>,
}
