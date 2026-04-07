use serde::{Deserialize, Serialize};
use tabled::Tabled;

fn display_option<T: std::fmt::Display>(o: &Option<T>) -> String {
    o.as_ref().map_or("-".to_string(), |v| v.to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct Guild {
    pub id: String,
    pub name: String,
    #[tabled(skip)]
    pub icon: Option<String>,
    #[tabled(display_with = "display_option")]
    pub owner_id: Option<String>,
    #[tabled(display_with = "display_option")]
    pub member_count: Option<u64>,
    #[tabled(display_with = "display_option")]
    pub description: Option<String>,
    #[tabled(skip)]
    #[serde(flatten)]
    pub extra: serde_json::Value,
}
