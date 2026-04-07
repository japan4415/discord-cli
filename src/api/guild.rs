use anyhow::Result;

use crate::client::DiscordClient;
use crate::models::channel::Channel;
use crate::models::guild::Guild;

impl DiscordClient {
    pub async fn get_guild(&self, guild_id: &str) -> Result<Guild> {
        self.get(&format!("/guilds/{}", guild_id)).await
    }

    pub async fn list_guilds(&self) -> Result<Vec<Guild>> {
        self.get("/users/@me/guilds").await
    }

    pub async fn create_guild(&self, params: &serde_json::Value) -> Result<Guild> {
        self.post("/guilds", params).await
    }

    pub async fn edit_guild(&self, guild_id: &str, params: &serde_json::Value) -> Result<Guild> {
        self.patch(&format!("/guilds/{}", guild_id), params).await
    }

    pub async fn delete_guild(&self, guild_id: &str) -> Result<()> {
        self.delete(&format!("/guilds/{}", guild_id)).await
    }

    pub async fn get_guild_preview(&self, guild_id: &str) -> Result<Guild> {
        self.get(&format!("/guilds/{}/preview", guild_id)).await
    }

    pub async fn get_guild_channels(&self, guild_id: &str) -> Result<Vec<Channel>> {
        self.get(&format!("/guilds/{}/channels", guild_id)).await
    }
}
