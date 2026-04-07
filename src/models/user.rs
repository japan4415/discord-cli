use serde::{Deserialize, Serialize};
use tabled::Tabled;

fn display_option<T: std::fmt::Display>(o: &Option<T>) -> String {
    o.as_ref().map_or("-".to_string(), |v| v.to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct User {
    pub id: String,
    pub username: String,
    #[tabled(display_with = "display_option")]
    pub discriminator: Option<String>,
    #[tabled(skip)]
    pub avatar: Option<String>,
    #[tabled(display_with = "display_option")]
    pub bot: Option<bool>,
    #[tabled(display_with = "display_option")]
    pub global_name: Option<String>,
    #[tabled(skip)]
    #[serde(flatten)]
    pub extra: serde_json::Value,
}
