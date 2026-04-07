use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub color: u32,
    pub hoist: bool,
    pub position: i32,
    pub permissions: String,
    pub mentionable: bool,
    #[tabled(skip)]
    #[serde(flatten)]
    pub extra: serde_json::Value,
}
