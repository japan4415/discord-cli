use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct Emoji {
    #[tabled(display_with = "display_option")]
    pub id: Option<String>,
    #[tabled(display_with = "display_option")]
    pub name: Option<String>,
    #[tabled(skip)]
    pub roles: Option<Vec<String>>,
    #[tabled(display_with = "display_option")]
    pub require_colons: Option<bool>,
    #[tabled(display_with = "display_option")]
    pub managed: Option<bool>,
    #[tabled(display_with = "display_option")]
    pub animated: Option<bool>,
    #[tabled(display_with = "display_option")]
    pub available: Option<bool>,
}
