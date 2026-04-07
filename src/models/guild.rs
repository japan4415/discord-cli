use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct Guild {
    pub id: String,
    pub name: String,
    #[tabled(skip)]
    pub icon: Option<String>,
    #[tabled(display = "display_option")]
    pub owner_id: Option<String>,
    #[tabled(display = "display_option")]
    pub member_count: Option<u64>,
    #[tabled(display = "display_option")]
    pub description: Option<String>,
    #[tabled(skip)]
    #[serde(flatten)]
    pub extra: serde_json::Value,
}
