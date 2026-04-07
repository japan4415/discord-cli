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
    params: &Value,
) -> Result<Sticker> {
    client
        .post(&format!("/guilds/{}/stickers", guild_id), params)
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
