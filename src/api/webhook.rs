use anyhow::Result;

use crate::client::DiscordClient;
use crate::models::webhook::Webhook;

impl DiscordClient {
    pub async fn list_channel_webhooks(&self, channel_id: &str) -> Result<Vec<Webhook>> {
        self.get(&format!("/channels/{}/webhooks", channel_id))
            .await
    }

    #[allow(dead_code)]
    pub async fn list_guild_webhooks(&self, guild_id: &str) -> Result<Vec<Webhook>> {
        self.get(&format!("/guilds/{}/webhooks", guild_id)).await
    }

    pub async fn get_webhook(&self, webhook_id: &str) -> Result<Webhook> {
        self.get(&format!("/webhooks/{}", webhook_id)).await
    }

    pub async fn create_webhook(
        &self,
        channel_id: &str,
        params: &serde_json::Value,
    ) -> Result<Webhook> {
        self.post(&format!("/channels/{}/webhooks", channel_id), params)
            .await
    }

    pub async fn edit_webhook(
        &self,
        webhook_id: &str,
        params: &serde_json::Value,
    ) -> Result<Webhook> {
        self.patch(&format!("/webhooks/{}", webhook_id), params)
            .await
    }

    pub async fn delete_webhook(&self, webhook_id: &str) -> Result<()> {
        self.delete(&format!("/webhooks/{}", webhook_id)).await
    }

    pub async fn execute_webhook(
        &self,
        webhook_id: &str,
        token: &str,
        params: &serde_json::Value,
    ) -> Result<()> {
        let _: serde_json::Value = self
            .post(
                &format!("/webhooks/{}/{}?wait=true", webhook_id, token),
                params,
            )
            .await?;
        Ok(())
    }
}
