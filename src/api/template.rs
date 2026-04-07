use anyhow::Result;
use serde_json::Value;

use crate::client::DiscordClient;
use crate::models::template::GuildTemplate;

pub async fn list_guild_templates(
    client: &DiscordClient,
    guild_id: &str,
) -> Result<Vec<GuildTemplate>> {
    client.get(&format!("/guilds/{}/templates", guild_id)).await
}

pub async fn get_template(client: &DiscordClient, code: &str) -> Result<GuildTemplate> {
    client.get(&format!("/guilds/templates/{}", code)).await
}

pub async fn create_guild_template(
    client: &DiscordClient,
    guild_id: &str,
    params: &Value,
) -> Result<GuildTemplate> {
    client
        .post(&format!("/guilds/{}/templates", guild_id), params)
        .await
}

pub async fn sync_guild_template(
    client: &DiscordClient,
    guild_id: &str,
    code: &str,
) -> Result<GuildTemplate> {
    client
        .put(
            &format!("/guilds/{}/templates/{}", guild_id, code),
            &serde_json::json!({}),
        )
        .await
}

pub async fn edit_guild_template(
    client: &DiscordClient,
    guild_id: &str,
    code: &str,
    params: &Value,
) -> Result<GuildTemplate> {
    client
        .patch(&format!("/guilds/{}/templates/{}", guild_id, code), params)
        .await
}

pub async fn delete_guild_template(
    client: &DiscordClient,
    guild_id: &str,
    code: &str,
) -> Result<()> {
    client
        .delete(&format!("/guilds/{}/templates/{}", guild_id, code))
        .await
}
