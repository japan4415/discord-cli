use anyhow::Result;

use crate::client::DiscordClient;
use crate::models::channel::Channel;

impl DiscordClient {
    pub async fn get_channel(&self, channel_id: &str) -> Result<Channel> {
        self.get(&format!("/channels/{}", channel_id)).await
    }

    pub async fn create_channel(
        &self,
        guild_id: &str,
        params: &serde_json::Value,
    ) -> Result<Channel> {
        self.post(&format!("/guilds/{}/channels", guild_id), params)
            .await
    }

    pub async fn edit_channel(
        &self,
        channel_id: &str,
        params: &serde_json::Value,
    ) -> Result<Channel> {
        self.patch(&format!("/channels/{}", channel_id), params)
            .await
    }

    pub async fn delete_channel(&self, channel_id: &str) -> Result<()> {
        self.delete(&format!("/channels/{}", channel_id)).await
    }
}
