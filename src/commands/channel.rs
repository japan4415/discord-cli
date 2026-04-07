use anyhow::Result;
use clap::Subcommand;

use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum ChannelCommand {
    /// Get a channel by ID
    Get {
        #[arg(long)]
        id: String,
    },
    /// List channels in a guild
    List {
        #[arg(long)]
        guild_id: String,
    },
    /// Create a channel in a guild
    Create {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        name: String,
        /// Channel type (0=text, 2=voice, 4=category, 5=announcement, 13=stage, 15=forum)
        #[arg(long, default_value = "0")]
        r#type: u8,
    },
    /// Edit a channel
    Edit {
        #[arg(long)]
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        topic: Option<String>,
    },
    /// Delete a channel
    Delete {
        #[arg(long)]
        id: String,
    },
}

impl ChannelCommand {
    pub async fn execute(self, client: &DiscordClient, output_format: &OutputFormat) -> Result<()> {
        match self {
            Self::Get { id } => {
                let channel = client.get_channel(&id).await?;
                output::render(output_format, &channel)?;
            }
            Self::List { guild_id } => {
                let channels = client.get_guild_channels(&guild_id).await?;
                output::render_list(output_format, &channels)?;
            }
            Self::Create {
                guild_id,
                name,
                r#type,
            } => {
                let params = serde_json::json!({ "name": name, "type": r#type });
                let channel = client.create_channel(&guild_id, &params).await?;
                output::render(output_format, &channel)?;
            }
            Self::Edit { id, name, topic } => {
                let mut params = serde_json::Map::new();
                if let Some(n) = name {
                    params.insert("name".into(), serde_json::Value::String(n));
                }
                if let Some(t) = topic {
                    params.insert("topic".into(), serde_json::Value::String(t));
                }
                let channel = client
                    .edit_channel(&id, &serde_json::Value::Object(params))
                    .await?;
                output::render(output_format, &channel)?;
            }
            Self::Delete { id } => {
                client.delete_channel(&id).await?;
                println!("Channel {} deleted.", id);
            }
        }
        Ok(())
    }
}
