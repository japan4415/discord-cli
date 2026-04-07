use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct StageInstance {
    pub id: String,
    pub guild_id: String,
    pub channel_id: String,
    pub topic: String,
    #[tabled(display_with = "display_option")]
    pub privacy_level: Option<u8>,
}
