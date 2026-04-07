use anyhow::Result;
use serde::Serialize;

use crate::client::DiscordClient;
use crate::models::audit_log::AuditLog;

#[derive(Debug, Serialize)]
pub struct AuditLogQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}

impl DiscordClient {
    pub async fn get_guild_audit_log(
        &self,
        guild_id: &str,
        query: &AuditLogQuery,
    ) -> Result<AuditLog> {
        self.get_with_query(&format!("/guilds/{}/audit-logs", guild_id), query)
            .await
    }
}
