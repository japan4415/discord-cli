use anyhow::Result;

use crate::client::DiscordClient;
use crate::models::invite::Invite;

impl DiscordClient {
    pub async fn get_invite(&self, code: &str) -> Result<Invite> {
        self.get(&format!("/invites/{}", code)).await
    }

    pub async fn list_guild_invites(&self, guild_id: &str) -> Result<Vec<Invite>> {
        self.get(&format!("/guilds/{}/invites", guild_id)).await
    }

    #[allow(dead_code)]
    pub async fn list_channel_invites(&self, channel_id: &str) -> Result<Vec<Invite>> {
        self.get(&format!("/channels/{}/invites", channel_id)).await
    }

    pub async fn create_channel_invite(
        &self,
        channel_id: &str,
        params: &serde_json::Value,
    ) -> Result<Invite> {
        self.post(&format!("/channels/{}/invites", channel_id), params)
            .await
    }

    pub async fn delete_invite(&self, code: &str) -> Result<()> {
        self.delete(&format!("/invites/{}", code)).await
    }
}
