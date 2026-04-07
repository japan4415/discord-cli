use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct Emoji {
    #[tabled(display = "display_option")]
    pub id: Option<String>,
    #[tabled(display = "display_option")]
    pub name: Option<String>,
    #[tabled(skip)]
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct Reaction {
    pub count: u64,
    #[tabled(skip)]
    pub emoji: Emoji,
    pub me: bool,
}
