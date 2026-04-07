use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::user::User;

fn display_option<T: std::fmt::Display>(o: &Option<T>) -> String {
    o.as_ref().map_or("-".to_string(), |v| v.to_string())
}

fn display_user(u: &User) -> String {
    format!("{}({})", u.username, u.id)
}

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct Ban {
    #[tabled(display_with = "display_option")]
    pub reason: Option<String>,
    #[tabled(display_with = "display_user")]
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkBanResponse {
    pub banned_users: Vec<String>,
    pub failed_users: Vec<String>,
}
