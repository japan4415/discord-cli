use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct Invite {
    pub code: String,
    #[tabled(skip)]
    pub guild: Option<serde_json::Value>,
    #[tabled(skip)]
    pub channel: Option<serde_json::Value>,
    #[tabled(skip)]
    pub inviter: Option<serde_json::Value>,
    #[tabled(display = "display_option")]
    pub uses: Option<u64>,
    #[tabled(display = "display_option")]
    pub max_uses: Option<u64>,
    #[tabled(display = "display_option")]
    pub max_age: Option<u64>,
    #[tabled(display = "display_option")]
    pub temporary: Option<bool>,
    #[tabled(display = "display_option")]
    pub created_at: Option<String>,
    #[tabled(display = "display_option")]
    pub expires_at: Option<String>,
}
