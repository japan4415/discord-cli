use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct VoiceRegion {
    pub id: String,
    pub name: String,
    #[tabled(display = "display_option")]
    pub optimal: Option<bool>,
    #[tabled(display = "display_option")]
    pub deprecated: Option<bool>,
    #[tabled(display = "display_option")]
    pub custom: Option<bool>,
}
