use anyhow::Result;
use clap::Subcommand;

use crate::api::stage;
use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum StageCommand {
    /// Get a stage instance by channel ID
    Get {
        #[arg(long)]
        id: String,
    },
    /// Create a stage instance
    Create {
        #[arg(long)]
        channel_id: String,
        #[arg(long)]
        topic: String,
        #[arg(long)]
        privacy_level: Option<u8>,
    },
    /// Edit a stage instance
    Edit {
        #[arg(long)]
        id: String,
        #[arg(long)]
        topic: String,
    },
    /// Delete a stage instance
    Delete {
        #[arg(long)]
        id: String,
    },
}

impl StageCommand {
    pub async fn execute(self, client: &DiscordClient, output_format: &OutputFormat) -> Result<()> {
        match self {
            Self::Get { id } => {
                let s = stage::get_stage(client, &id).await?;
                output::render(output_format, &s)?;
            }
            Self::Create {
                channel_id,
                topic,
                privacy_level,
            } => {
                let mut params = serde_json::json!({
                    "channel_id": channel_id,
                    "topic": topic,
                });
                if let Some(level) = privacy_level {
                    params["privacy_level"] = serde_json::Value::Number(level.into());
                }
                let s = stage::create_stage(client, &params).await?;
                output::render(output_format, &s)?;
            }
            Self::Edit { id, topic } => {
                let params = serde_json::json!({ "topic": topic });
                let s = stage::edit_stage(client, &id, &params).await?;
                output::render(output_format, &s)?;
            }
            Self::Delete { id } => {
                stage::delete_stage(client, &id).await?;
                println!("Stage instance deleted successfully.");
            }
        }
        Ok(())
    }
}
