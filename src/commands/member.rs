use anyhow::Result;
use clap::Subcommand;

use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum MemberCommand {
    /// Get a guild member
    Get {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        user_id: String,
    },
    /// List guild members
    List {
        #[arg(long)]
        guild_id: String,
        #[arg(long, default_value = "100")]
        limit: u64,
    },
    /// Search guild members by name
    Search {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        query: String,
    },
    /// Kick a member from a guild
    Kick {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        user_id: String,
    },
    /// Edit a guild member
    Edit {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        user_id: String,
        #[arg(long)]
        nick: Option<String>,
    },
}

impl MemberCommand {
    pub async fn execute(self, client: &DiscordClient, output_format: &OutputFormat) -> Result<()> {
        match self {
            Self::Get { guild_id, user_id } => {
                let member = client.get_member(&guild_id, &user_id).await?;
                output::render(output_format, &member)?;
            }
            Self::List { guild_id, limit } => {
                let members = client.list_members(&guild_id, Some(limit)).await?;
                output::render_list(output_format, &members)?;
            }
            Self::Search { guild_id, query } => {
                let members = client.search_members(&guild_id, &query).await?;
                output::render_list(output_format, &members)?;
            }
            Self::Kick { guild_id, user_id } => {
                client.kick_member(&guild_id, &user_id).await?;
                println!("Member {} kicked from guild {}.", user_id, guild_id);
            }
            Self::Edit {
                guild_id,
                user_id,
                nick,
            } => {
                let mut params = serde_json::Map::new();
                if let Some(n) = nick {
                    params.insert("nick".into(), serde_json::Value::String(n));
                }
                let member = client
                    .edit_member(&guild_id, &user_id, &serde_json::Value::Object(params))
                    .await?;
                output::render(output_format, &member)?;
            }
        }
        Ok(())
    }
}
