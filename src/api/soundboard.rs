use anyhow::Result;

use crate::client::DiscordClient;
use crate::models::soundboard::SoundboardSound;

pub async fn list_default_soundboard_sounds(
    client: &DiscordClient,
) -> Result<Vec<SoundboardSound>> {
    client.get("/soundboard-default-sounds").await
}

pub async fn list_guild_soundboard_sounds(
    client: &DiscordClient,
    guild_id: &str,
) -> Result<Vec<SoundboardSound>> {
    client
        .get(&format!("/guilds/{}/soundboard-sounds", guild_id))
        .await
}
