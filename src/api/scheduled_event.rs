use anyhow::Result;
use serde_json::Value;

use crate::client::DiscordClient;
use crate::models::scheduled_event::{EventUser, GuildScheduledEvent};

pub async fn list_events(
    client: &DiscordClient,
    guild_id: &str,
) -> Result<Vec<GuildScheduledEvent>> {
    client
        .get(&format!("/guilds/{}/scheduled-events", guild_id))
        .await
}

pub async fn get_event(
    client: &DiscordClient,
    guild_id: &str,
    event_id: &str,
) -> Result<GuildScheduledEvent> {
    client
        .get(&format!(
            "/guilds/{}/scheduled-events/{}",
            guild_id, event_id
        ))
        .await
}

pub async fn create_event(
    client: &DiscordClient,
    guild_id: &str,
    params: &Value,
) -> Result<GuildScheduledEvent> {
    client
        .post(&format!("/guilds/{}/scheduled-events", guild_id), params)
        .await
}

pub async fn edit_event(
    client: &DiscordClient,
    guild_id: &str,
    event_id: &str,
    params: &Value,
) -> Result<GuildScheduledEvent> {
    client
        .patch(
            &format!("/guilds/{}/scheduled-events/{}", guild_id, event_id),
            params,
        )
        .await
}

pub async fn delete_event(client: &DiscordClient, guild_id: &str, event_id: &str) -> Result<()> {
    client
        .delete(&format!(
            "/guilds/{}/scheduled-events/{}",
            guild_id, event_id
        ))
        .await
}

pub async fn list_event_users(
    client: &DiscordClient,
    guild_id: &str,
    event_id: &str,
) -> Result<Vec<EventUser>> {
    client
        .get(&format!(
            "/guilds/{}/scheduled-events/{}/users",
            guild_id, event_id
        ))
        .await
}
