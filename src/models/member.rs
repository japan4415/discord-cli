use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;
use super::user::User;

fn display_user(u: &Option<User>) -> String {
    u.as_ref()
        .map_or("-".to_string(), |u| format!("{}({})", u.username, u.id))
}

fn display_roles(roles: &[String]) -> String {
    if roles.is_empty() {
        "-".to_string()
    } else {
        roles.join(", ")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct GuildMember {
    #[tabled(display_with = "display_user")]
    pub user: Option<User>,
    #[tabled(display_with = "display_option")]
    pub nick: Option<String>,
    #[tabled(display_with = "display_roles")]
    pub roles: Vec<String>,
    #[tabled(display_with = "display_option")]
    pub joined_at: Option<String>,
    #[tabled(skip)]
    #[serde(flatten)]
    pub extra: serde_json::Value,
}
