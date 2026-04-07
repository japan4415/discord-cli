use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct Sticker {
    pub id: String,
    pub name: String,
    #[tabled(display_with = "display_option")]
    pub description: Option<String>,
    #[tabled(skip)]
    pub tags: Option<String>,
    #[serde(rename = "type")]
    #[tabled(display_with = "display_option")]
    pub sticker_type: Option<u8>,
    #[tabled(display_with = "display_option")]
    pub format_type: Option<u8>,
    #[tabled(display_with = "display_option")]
    pub guild_id: Option<String>,
    #[tabled(display_with = "display_option")]
    pub available: Option<bool>,
}
