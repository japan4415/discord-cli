use anyhow::Result;
use clap::Subcommand;

use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum GuildCommand {
    /// Get a guild by ID
    Get {
        #[arg(long)]
        id: String,
    },
    /// List guilds the bot/user is in
    List,
    /// Create a new guild
    Create {
        #[arg(long)]
        name: String,
    },
    /// Edit a guild
    Edit {
        #[arg(long)]
        id: String,
        #[arg(long)]
        name: Option<String>,
    },
    /// Delete a guild
    Delete {
        #[arg(long)]
        id: String,
    },
    /// Get a guild preview
    Preview {
        #[arg(long)]
        id: String,
    },
}

impl GuildCommand {
    pub async fn execute(self, client: &DiscordClient, output_format: &OutputFormat) -> Result<()> {
        match self {
            Self::Get { id } => {
                let guild = client.get_guild(&id).await?;
                output::render(output_format, &guild)?;
            }
            Self::List => {
                let guilds = client.list_guilds().await?;
                output::render_list(output_format, &guilds)?;
            }
            Self::Create { name } => {
                let params = serde_json::json!({ "name": name });
                let guild = client.create_guild(&params).await?;
                output::render(output_format, &guild)?;
            }
            Self::Edit { id, name } => {
                let mut params = serde_json::Map::new();
                if let Some(n) = name {
                    params.insert("name".into(), serde_json::Value::String(n));
                }
                let guild = client
                    .edit_guild(&id, &serde_json::Value::Object(params))
                    .await?;
                output::render(output_format, &guild)?;
            }
            Self::Delete { id } => {
                client.delete_guild(&id).await?;
                println!("Guild {} deleted.", id);
            }
            Self::Preview { id } => {
                let guild = client.get_guild_preview(&id).await?;
                output::render(output_format, &guild)?;
            }
        }
        Ok(())
    }
}
