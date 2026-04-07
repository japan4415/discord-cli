use anyhow::Result;

use crate::client::DiscordClient;
use crate::models::pin::PinnedMessage;

impl DiscordClient {
    pub async fn list_pins(&self, channel_id: &str) -> Result<Vec<PinnedMessage>> {
        self.get(&format!("/channels/{}/pins", channel_id)).await
    }

    pub async fn pin_message(&self, channel_id: &str, message_id: &str) -> Result<()> {
        self.put_empty(&format!("/channels/{}/pins/{}", channel_id, message_id))
            .await
    }

    pub async fn unpin_message(&self, channel_id: &str, message_id: &str) -> Result<()> {
        self.delete(&format!("/channels/{}/pins/{}", channel_id, message_id))
            .await
    }
}
