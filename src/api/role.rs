use anyhow::Result;

use crate::client::DiscordClient;
use crate::models::role::Role;

impl DiscordClient {
    pub async fn list_roles(&self, guild_id: &str) -> Result<Vec<Role>> {
        self.get(&format!("/guilds/{}/roles", guild_id)).await
    }

    pub async fn create_role(&self, guild_id: &str, params: &serde_json::Value) -> Result<Role> {
        self.post(&format!("/guilds/{}/roles", guild_id), params)
            .await
    }

    pub async fn edit_role(
        &self,
        guild_id: &str,
        role_id: &str,
        params: &serde_json::Value,
    ) -> Result<Role> {
        self.patch(&format!("/guilds/{}/roles/{}", guild_id, role_id), params)
            .await
    }

    pub async fn delete_role(&self, guild_id: &str, role_id: &str) -> Result<()> {
        self.delete(&format!("/guilds/{}/roles/{}", guild_id, role_id))
            .await
    }

    pub async fn add_role_to_member(
        &self,
        guild_id: &str,
        user_id: &str,
        role_id: &str,
    ) -> Result<()> {
        self.put_empty(&format!(
            "/guilds/{}/members/{}/roles/{}",
            guild_id, user_id, role_id
        ))
        .await
    }

    pub async fn remove_role_from_member(
        &self,
        guild_id: &str,
        user_id: &str,
        role_id: &str,
    ) -> Result<()> {
        self.delete(&format!(
            "/guilds/{}/members/{}/roles/{}",
            guild_id, user_id, role_id
        ))
        .await
    }
}
