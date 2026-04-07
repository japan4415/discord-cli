use anyhow::Result;
use clap::Subcommand;

use crate::api::auto_moderation;
use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum AutoModCommand {
    /// List auto-moderation rules in a guild
    List {
        #[arg(long)]
        guild_id: String,
    },
    /// Get a specific auto-moderation rule
    Get {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        id: String,
    },
    /// Create an auto-moderation rule (JSON params)
    Create {
        #[arg(long)]
        guild_id: String,
        /// JSON data for rule creation
        #[arg(long)]
        json: String,
    },
    /// Edit an auto-moderation rule (JSON params)
    Edit {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        id: String,
        /// JSON data for rule update
        #[arg(long)]
        json: String,
    },
    /// Delete an auto-moderation rule
    Delete {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        id: String,
    },
}

impl AutoModCommand {
    pub async fn execute(self, client: &DiscordClient, output_format: &OutputFormat) -> Result<()> {
        match self {
            Self::List { guild_id } => {
                let rules = auto_moderation::list_rules(client, &guild_id).await?;
                output::render_list(output_format, &rules)?;
            }
            Self::Get { guild_id, id } => {
                let rule = auto_moderation::get_rule(client, &guild_id, &id).await?;
                output::render(output_format, &rule)?;
            }
            Self::Create { guild_id, json } => {
                let params: serde_json::Value = serde_json::from_str(&json)?;
                let rule = auto_moderation::create_rule(client, &guild_id, &params).await?;
                output::render(output_format, &rule)?;
            }
            Self::Edit { guild_id, id, json } => {
                let params: serde_json::Value = serde_json::from_str(&json)?;
                let rule = auto_moderation::edit_rule(client, &guild_id, &id, &params).await?;
                output::render(output_format, &rule)?;
            }
            Self::Delete { guild_id, id } => {
                auto_moderation::delete_rule(client, &guild_id, &id).await?;
                println!("Auto-moderation rule deleted successfully.");
            }
        }
        Ok(())
    }
}
