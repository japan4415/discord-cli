use anyhow::Result;

use crate::client::DiscordClient;
use crate::models::ban::{Ban, BulkBanResponse};

impl DiscordClient {
    pub async fn list_bans(&self, guild_id: &str) -> Result<Vec<Ban>> {
        self.get(&format!("/guilds/{}/bans", guild_id)).await
    }

    pub async fn get_ban(&self, guild_id: &str, user_id: &str) -> Result<Ban> {
        self.get(&format!("/guilds/{}/bans/{}", guild_id, user_id))
            .await
    }

    pub async fn create_ban(
        &self,
        guild_id: &str,
        user_id: &str,
        params: &serde_json::Value,
    ) -> Result<()> {
        self.put_empty_with_body(&format!("/guilds/{}/bans/{}", guild_id, user_id), params)
            .await
    }

    pub async fn remove_ban(&self, guild_id: &str, user_id: &str) -> Result<()> {
        self.delete(&format!("/guilds/{}/bans/{}", guild_id, user_id))
            .await
    }

    pub async fn bulk_ban(
        &self,
        guild_id: &str,
        params: &serde_json::Value,
    ) -> Result<BulkBanResponse> {
        self.post(&format!("/guilds/{}/bulk-ban", guild_id), params)
            .await
    }
}
