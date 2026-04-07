use anyhow::Result;
use clap::Subcommand;

use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum BanCommand {
    /// List bans in a guild
    List {
        #[arg(long)]
        guild_id: String,
    },
    /// Get a ban for a user
    Get {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        user_id: String,
    },
    /// Ban a user from a guild
    Create {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        user_id: String,
        #[arg(long)]
        reason: Option<String>,
        /// Number of seconds to delete messages for (0-604800)
        #[arg(long)]
        delete_message_seconds: Option<u64>,
    },
    /// Remove a ban from a user
    Remove {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        user_id: String,
    },
    /// Bulk ban users from a guild
    Bulk {
        #[arg(long)]
        guild_id: String,
        /// Comma-separated user IDs
        #[arg(long, value_delimiter = ',')]
        user_ids: Vec<String>,
    },
}

impl BanCommand {
    pub async fn execute(self, client: &DiscordClient, output_format: &OutputFormat) -> Result<()> {
        match self {
            Self::List { guild_id } => {
                let bans = client.list_bans(&guild_id).await?;
                output::render_list(output_format, &bans)?;
            }
            Self::Get { guild_id, user_id } => {
                let ban = client.get_ban(&guild_id, &user_id).await?;
                output::render(output_format, &ban)?;
            }
            Self::Create {
                guild_id,
                user_id,
                reason,
                delete_message_seconds,
            } => {
                let mut params = serde_json::Map::new();
                if let Some(s) = delete_message_seconds {
                    params.insert("delete_message_seconds".into(), serde_json::json!(s));
                }
                client
                    .create_ban(
                        &guild_id,
                        &user_id,
                        &serde_json::Value::Object(params),
                        reason.as_deref(),
                    )
                    .await?;
                println!("User {} banned from guild {}.", user_id, guild_id);
            }
            Self::Remove { guild_id, user_id } => {
                client.remove_ban(&guild_id, &user_id).await?;
                println!("Ban removed for user {} in guild {}.", user_id, guild_id);
            }
            Self::Bulk { guild_id, user_ids } => {
                let params = serde_json::json!({ "user_ids": user_ids });
                let response = client.bulk_ban(&guild_id, &params).await?;
                println!(
                    "Banned: {} users, Failed: {} users",
                    response.banned_users.len(),
                    response.failed_users.len()
                );
            }
        }
        Ok(())
    }
}
