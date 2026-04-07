use anyhow::Result;

use crate::client::DiscordClient;
use crate::models::user::User;

impl DiscordClient {
    pub async fn add_reaction(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji: &str,
    ) -> Result<()> {
        let encoded = urlencoding::encode(emoji);
        self.put_empty(&format!(
            "/channels/{}/messages/{}/reactions/{}/@me",
            channel_id, message_id, encoded
        ))
        .await
    }

    pub async fn remove_own_reaction(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji: &str,
    ) -> Result<()> {
        let encoded = urlencoding::encode(emoji);
        self.delete(&format!(
            "/channels/{}/messages/{}/reactions/{}/@me",
            channel_id, message_id, encoded
        ))
        .await
    }

    #[allow(dead_code)]
    pub async fn remove_user_reaction(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji: &str,
        user_id: &str,
    ) -> Result<()> {
        let encoded = urlencoding::encode(emoji);
        self.delete(&format!(
            "/channels/{}/messages/{}/reactions/{}/{}",
            channel_id, message_id, encoded, user_id
        ))
        .await
    }

    pub async fn list_reactions(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji: &str,
    ) -> Result<Vec<User>> {
        let encoded = urlencoding::encode(emoji);
        self.get(&format!(
            "/channels/{}/messages/{}/reactions/{}",
            channel_id, message_id, encoded
        ))
        .await
    }

    pub async fn clear_all_reactions(&self, channel_id: &str, message_id: &str) -> Result<()> {
        self.delete(&format!(
            "/channels/{}/messages/{}/reactions",
            channel_id, message_id
        ))
        .await
    }

    pub async fn clear_emoji_reactions(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji: &str,
    ) -> Result<()> {
        let encoded = urlencoding::encode(emoji);
        self.delete(&format!(
            "/channels/{}/messages/{}/reactions/{}",
            channel_id, message_id, encoded
        ))
        .await
    }
}
