use anyhow::Result;
use serde_json::Value;

use crate::client::DiscordClient;
use crate::models::sticker::Sticker;

pub async fn list_guild_stickers(client: &DiscordClient, guild_id: &str) -> Result<Vec<Sticker>> {
    client.get(&format!("/guilds/{}/stickers", guild_id)).await
}

pub async fn get_guild_sticker(
    client: &DiscordClient,
    guild_id: &str,
    sticker_id: &str,
) -> Result<Sticker> {
    client
        .get(&format!("/guilds/{}/stickers/{}", guild_id, sticker_id))
        .await
}

pub async fn create_guild_sticker(
    client: &DiscordClient,
    guild_id: &str,
    name: &str,
    description: &str,
    tags: &str,
    file_path: &str,
) -> Result<Sticker> {
    let file_bytes = tokio::fs::read(file_path).await?;
    let file_name = std::path::Path::new(file_path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let part = reqwest::multipart::Part::bytes(file_bytes).file_name(file_name);

    let form = reqwest::multipart::Form::new()
        .text("name", name.to_string())
        .text("description", description.to_string())
        .text("tags", tags.to_string())
        .part("file", part);

    client
        .post_multipart(&format!("/guilds/{}/stickers", guild_id), form)
        .await
}

pub async fn edit_guild_sticker(
    client: &DiscordClient,
    guild_id: &str,
    sticker_id: &str,
    params: &Value,
) -> Result<Sticker> {
    client
        .patch(
            &format!("/guilds/{}/stickers/{}", guild_id, sticker_id),
            params,
        )
        .await
}

pub async fn delete_guild_sticker(
    client: &DiscordClient,
    guild_id: &str,
    sticker_id: &str,
) -> Result<()> {
    client
        .delete(&format!("/guilds/{}/stickers/{}", guild_id, sticker_id))
        .await
}
