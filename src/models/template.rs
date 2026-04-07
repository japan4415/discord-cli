use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct GuildTemplate {
    pub code: String,
    pub name: String,
    #[tabled(display = "display_option")]
    pub description: Option<String>,
    #[tabled(display = "display_option")]
    pub usage_count: Option<u64>,
    #[tabled(display = "display_option")]
    pub creator_id: Option<String>,
    #[tabled(display = "display_option")]
    pub source_guild_id: Option<String>,
}
