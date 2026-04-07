use anyhow::Result;

use crate::client::DiscordClient;
use crate::models::message::Message;

impl DiscordClient {
    pub async fn get_message(&self, channel_id: &str, message_id: &str) -> Result<Message> {
        self.get(&format!("/channels/{}/messages/{}", channel_id, message_id))
            .await
    }

    pub async fn list_messages(
        &self,
        channel_id: &str,
        limit: Option<u64>,
        before: Option<&str>,
        after: Option<&str>,
    ) -> Result<Vec<Message>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        if let Some(b) = before {
            query.push(("before", b.to_string()));
        }
        if let Some(a) = after {
            query.push(("after", a.to_string()));
        }
        self.get_with_query(&format!("/channels/{}/messages", channel_id), &query)
            .await
    }

    pub async fn send_message(
        &self,
        channel_id: &str,
        params: &serde_json::Value,
    ) -> Result<Message> {
        self.post(&format!("/channels/{}/messages", channel_id), params)
            .await
    }

    pub async fn edit_message(
        &self,
        channel_id: &str,
        message_id: &str,
        params: &serde_json::Value,
    ) -> Result<Message> {
        self.patch(
            &format!("/channels/{}/messages/{}", channel_id, message_id),
            params,
        )
        .await
    }

    pub async fn delete_message(&self, channel_id: &str, message_id: &str) -> Result<()> {
        self.delete(&format!("/channels/{}/messages/{}", channel_id, message_id))
            .await
    }

    pub async fn bulk_delete_messages(
        &self,
        channel_id: &str,
        message_ids: &[String],
    ) -> Result<()> {
        let body = serde_json::json!({ "messages": message_ids });
        self.post_empty_with_body(
            &format!("/channels/{}/messages/bulk-delete", channel_id),
            &body,
        )
        .await
    }

    pub async fn crosspost_message(&self, channel_id: &str, message_id: &str) -> Result<Message> {
        self.post(
            &format!("/channels/{}/messages/{}/crosspost", channel_id, message_id),
            &serde_json::json!({}),
        )
        .await
    }
}
