use anyhow::Result;

use crate::client::DiscordClient;
use crate::models::gateway::{GatewayBotInfo, GatewayInfo};

pub async fn get_gateway(client: &DiscordClient) -> Result<GatewayInfo> {
    client.get("/gateway").await
}

pub async fn get_gateway_bot(client: &DiscordClient) -> Result<GatewayBotInfo> {
    client.get("/gateway/bot").await
}
