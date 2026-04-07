use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;
use super::user::User;

fn display_user(u: &User) -> String {
    format!("{}({})", u.username, u.id)
}

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct Ban {
    #[tabled(display = "display_option")]
    pub reason: Option<String>,
    #[tabled(display = "display_user")]
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkBanResponse {
    pub banned_users: Vec<String>,
    pub failed_users: Vec<String>,
}
