use anyhow::Result;
use serde_json::Value;

use crate::client::DiscordClient;
use crate::models::stage::StageInstance;

pub async fn get_stage(client: &DiscordClient, channel_id: &str) -> Result<StageInstance> {
    client
        .get(&format!("/stage-instances/{}", channel_id))
        .await
}

pub async fn create_stage(client: &DiscordClient, params: &Value) -> Result<StageInstance> {
    client.post("/stage-instances", params).await
}

pub async fn edit_stage(
    client: &DiscordClient,
    channel_id: &str,
    params: &Value,
) -> Result<StageInstance> {
    client
        .patch(&format!("/stage-instances/{}", channel_id), params)
        .await
}

pub async fn delete_stage(client: &DiscordClient, channel_id: &str) -> Result<()> {
    client
        .delete(&format!("/stage-instances/{}", channel_id))
        .await
}
