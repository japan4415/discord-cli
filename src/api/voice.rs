use anyhow::Result;

use crate::client::DiscordClient;
use crate::models::voice::VoiceRegion;

pub async fn list_voice_regions(client: &DiscordClient) -> Result<Vec<VoiceRegion>> {
    client.get("/voice/regions").await
}
