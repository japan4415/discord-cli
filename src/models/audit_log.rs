use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub audit_log_entries: Vec<AuditLogEntry>,
    #[serde(default)]
    pub users: Vec<serde_json::Value>,
    #[serde(default)]
    pub integrations: Vec<serde_json::Value>,
    #[serde(default)]
    pub webhooks: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct AuditLogEntry {
    pub id: String,
    #[tabled(display = "display_option")]
    pub target_id: Option<String>,
    #[tabled(display = "display_option")]
    pub user_id: Option<String>,
    pub action_type: u64,
    #[tabled(display = "display_option")]
    pub reason: Option<String>,
}
