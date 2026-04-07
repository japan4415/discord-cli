use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct User {
    pub id: String,
    pub username: String,
    #[tabled(display = "display_option")]
    pub discriminator: Option<String>,
    #[tabled(skip)]
    pub avatar: Option<String>,
    #[tabled(display = "display_option")]
    pub bot: Option<bool>,
    #[tabled(display = "display_option")]
    pub global_name: Option<String>,
    #[tabled(skip)]
    #[serde(flatten)]
    pub extra: serde_json::Value,
}
