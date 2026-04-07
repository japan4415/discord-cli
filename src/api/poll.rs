use anyhow::Result;

use crate::client::DiscordClient;
use crate::models::poll::{Poll, PollVotersResponse};

pub async fn get_answer_voters(
    client: &DiscordClient,
    channel_id: &str,
    message_id: &str,
    answer_id: &str,
    after: Option<&str>,
    limit: Option<u64>,
) -> Result<PollVotersResponse> {
    let path = format!(
        "/channels/{}/polls/{}/answers/{}",
        channel_id, message_id, answer_id
    );

    let mut query: Vec<(String, String)> = Vec::new();
    if let Some(after) = after {
        query.push(("after".to_string(), after.to_string()));
    }
    if let Some(limit) = limit {
        query.push(("limit".to_string(), limit.to_string()));
    }

    if query.is_empty() {
        client.get(&path).await
    } else {
        client.get_with_query(&path, &query).await
    }
}

pub async fn expire_poll(
    client: &DiscordClient,
    channel_id: &str,
    message_id: &str,
) -> Result<Poll> {
    client
        .post(
            &format!("/channels/{}/polls/{}/expire", channel_id, message_id),
            &serde_json::json!({}),
        )
        .await
}
