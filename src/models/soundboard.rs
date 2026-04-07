use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct SoundboardSound {
    pub sound_id: String,
    pub name: String,
    #[tabled(display_with = "display_option")]
    pub volume: Option<f64>,
    #[tabled(display_with = "display_option")]
    pub emoji_id: Option<String>,
    #[tabled(display_with = "display_option")]
    pub emoji_name: Option<String>,
    #[tabled(display_with = "display_option")]
    pub guild_id: Option<String>,
    #[tabled(display_with = "display_option")]
    pub available: Option<bool>,
}
