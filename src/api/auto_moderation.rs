use anyhow::Result;
use serde_json::Value;

use crate::client::DiscordClient;
use crate::models::auto_moderation::AutoModerationRule;

pub async fn list_rules(client: &DiscordClient, guild_id: &str) -> Result<Vec<AutoModerationRule>> {
    client
        .get(&format!("/guilds/{}/auto-moderation/rules", guild_id))
        .await
}

pub async fn get_rule(
    client: &DiscordClient,
    guild_id: &str,
    rule_id: &str,
) -> Result<AutoModerationRule> {
    client
        .get(&format!(
            "/guilds/{}/auto-moderation/rules/{}",
            guild_id, rule_id
        ))
        .await
}

pub async fn create_rule(
    client: &DiscordClient,
    guild_id: &str,
    params: &Value,
) -> Result<AutoModerationRule> {
    client
        .post(
            &format!("/guilds/{}/auto-moderation/rules", guild_id),
            params,
        )
        .await
}

pub async fn edit_rule(
    client: &DiscordClient,
    guild_id: &str,
    rule_id: &str,
    params: &Value,
) -> Result<AutoModerationRule> {
    client
        .patch(
            &format!("/guilds/{}/auto-moderation/rules/{}", guild_id, rule_id),
            params,
        )
        .await
}

pub async fn delete_rule(client: &DiscordClient, guild_id: &str, rule_id: &str) -> Result<()> {
    client
        .delete(&format!(
            "/guilds/{}/auto-moderation/rules/{}",
            guild_id, rule_id
        ))
        .await
}
