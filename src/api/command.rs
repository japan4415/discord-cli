use anyhow::Result;

use crate::client::DiscordClient;
use crate::models::application::ApplicationCommand;

impl DiscordClient {
    pub async fn list_global_commands(&self, app_id: &str) -> Result<Vec<ApplicationCommand>> {
        self.get(&format!("/applications/{}/commands", app_id))
            .await
    }

    pub async fn list_guild_commands(
        &self,
        app_id: &str,
        guild_id: &str,
    ) -> Result<Vec<ApplicationCommand>> {
        self.get(&format!(
            "/applications/{}/guilds/{}/commands",
            app_id, guild_id
        ))
        .await
    }

    pub async fn create_global_command(
        &self,
        app_id: &str,
        params: &serde_json::Value,
    ) -> Result<ApplicationCommand> {
        self.post(&format!("/applications/{}/commands", app_id), params)
            .await
    }

    pub async fn create_guild_command(
        &self,
        app_id: &str,
        guild_id: &str,
        params: &serde_json::Value,
    ) -> Result<ApplicationCommand> {
        self.post(
            &format!("/applications/{}/guilds/{}/commands", app_id, guild_id),
            params,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn get_global_command(
        &self,
        app_id: &str,
        cmd_id: &str,
    ) -> Result<ApplicationCommand> {
        self.get(&format!("/applications/{}/commands/{}", app_id, cmd_id))
            .await
    }

    #[allow(dead_code)]
    pub async fn edit_global_command(
        &self,
        app_id: &str,
        cmd_id: &str,
        params: &serde_json::Value,
    ) -> Result<ApplicationCommand> {
        self.patch(
            &format!("/applications/{}/commands/{}", app_id, cmd_id),
            params,
        )
        .await
    }

    pub async fn delete_global_command(&self, app_id: &str, cmd_id: &str) -> Result<()> {
        self.delete(&format!("/applications/{}/commands/{}", app_id, cmd_id))
            .await
    }

    pub async fn delete_guild_command(
        &self,
        app_id: &str,
        guild_id: &str,
        cmd_id: &str,
    ) -> Result<()> {
        self.delete(&format!(
            "/applications/{}/guilds/{}/commands/{}",
            app_id, guild_id, cmd_id
        ))
        .await
    }
}
