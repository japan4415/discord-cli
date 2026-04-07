use anyhow::Result;

use crate::client::DiscordClient;
use crate::models::user::User;

impl DiscordClient {
    pub async fn get_user(&self, user_id: &str) -> Result<User> {
        self.get(&format!("/users/{}", user_id)).await
    }

    pub async fn get_current_user(&self) -> Result<User> {
        self.get("/users/@me").await
    }
}
