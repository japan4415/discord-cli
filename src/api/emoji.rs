use anyhow::Result;

use crate::client::DiscordClient;
use crate::models::emoji::Emoji;

impl DiscordClient {
    pub async fn list_guild_emojis(&self, guild_id: &str) -> Result<Vec<Emoji>> {
        self.get(&format!("/guilds/{}/emojis", guild_id)).await
    }

    pub async fn get_guild_emoji(&self, guild_id: &str, emoji_id: &str) -> Result<Emoji> {
        self.get(&format!("/guilds/{}/emojis/{}", guild_id, emoji_id))
            .await
    }

    pub async fn create_guild_emoji(
        &self,
        guild_id: &str,
        params: &serde_json::Value,
    ) -> Result<Emoji> {
        self.post(&format!("/guilds/{}/emojis", guild_id), params)
            .await
    }

    pub async fn edit_guild_emoji(
        &self,
        guild_id: &str,
        emoji_id: &str,
        params: &serde_json::Value,
    ) -> Result<Emoji> {
        self.patch(&format!("/guilds/{}/emojis/{}", guild_id, emoji_id), params)
            .await
    }

    pub async fn delete_guild_emoji(&self, guild_id: &str, emoji_id: &str) -> Result<()> {
        self.delete(&format!("/guilds/{}/emojis/{}", guild_id, emoji_id))
            .await
    }
}
