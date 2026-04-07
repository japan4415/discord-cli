use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[allow(dead_code)]
fn display_option<T: std::fmt::Display>(o: &Option<T>) -> String {
    o.as_ref().map_or("-".to_string(), |v| v.to_string())
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct Emoji {
    #[tabled(display_with = "display_option")]
    pub id: Option<String>,
    #[tabled(display_with = "display_option")]
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
