use serde::{Deserialize, Serialize};
use tabled::Tabled;

fn display_option<T: std::fmt::Display>(o: &Option<T>) -> String {
    o.as_ref().map_or("-".to_string(), |v| v.to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct Channel {
    pub id: String,
    #[serde(rename = "type")]
    pub channel_type: u8,
    #[tabled(display_with = "display_option")]
    pub guild_id: Option<String>,
    #[tabled(display_with = "display_option")]
    pub name: Option<String>,
    #[tabled(display_with = "display_option")]
    pub topic: Option<String>,
    #[tabled(display_with = "display_option")]
    pub nsfw: Option<bool>,
    #[tabled(display_with = "display_option")]
    pub position: Option<i32>,
    #[tabled(skip)]
    #[serde(flatten)]
    pub extra: serde_json::Value,
}
