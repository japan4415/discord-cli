use anyhow::Result;
use clap::Subcommand;

use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum CommandCommand {
    /// List global application commands
    ListGlobal {
        /// Application ID
        app_id: String,
    },
    /// List guild application commands
    ListGuild {
        /// Application ID
        app_id: String,
        /// Guild ID
        guild_id: String,
    },
    /// Create a global application command
    CreateGlobal {
        /// Application ID
        app_id: String,
        /// Command JSON definition
        json: String,
    },
    /// Create a guild application command
    CreateGuild {
        /// Application ID
        app_id: String,
        /// Guild ID
        guild_id: String,
        /// Command JSON definition
        json: String,
    },
    /// Delete a global application command
    DeleteGlobal {
        /// Application ID
        app_id: String,
        /// Command ID
        id: String,
    },
    /// Delete a guild application command
    DeleteGuild {
        /// Application ID
        app_id: String,
        /// Guild ID
        guild_id: String,
        /// Command ID
        id: String,
    },
}

pub async fn execute(
    client: &DiscordClient,
    command: CommandCommand,
    output_format: &OutputFormat,
) -> Result<()> {
    match command {
        CommandCommand::ListGlobal { app_id } => {
            let commands = client.list_global_commands(&app_id).await?;
            output::render_list(output_format, &commands)?;
        }
        CommandCommand::ListGuild { app_id, guild_id } => {
            let commands = client.list_guild_commands(&app_id, &guild_id).await?;
            output::render_list(output_format, &commands)?;
        }
        CommandCommand::CreateGlobal { app_id, json } => {
            let params: serde_json::Value = serde_json::from_str(&json)?;
            let cmd = client.create_global_command(&app_id, &params).await?;
            output::render(output_format, &cmd)?;
        }
        CommandCommand::CreateGuild {
            app_id,
            guild_id,
            json,
        } => {
            let params: serde_json::Value = serde_json::from_str(&json)?;
            let cmd = client
                .create_guild_command(&app_id, &guild_id, &params)
                .await?;
            output::render(output_format, &cmd)?;
        }
        CommandCommand::DeleteGlobal { app_id, id } => {
            client.delete_global_command(&app_id, &id).await?;
            println!("Global command {} deleted.", id);
        }
        CommandCommand::DeleteGuild {
            app_id,
            guild_id,
            id,
        } => {
            client.delete_guild_command(&app_id, &guild_id, &id).await?;
            println!("Guild command {} deleted.", id);
        }
    }
    Ok(())
}
