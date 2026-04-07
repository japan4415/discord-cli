use anyhow::Result;
use clap::Subcommand;

use crate::api::sticker;
use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum StickerCommand {
    /// List all stickers in a guild
    List {
        #[arg(long)]
        guild_id: String,
    },
    /// Get a specific guild sticker
    Get {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        id: String,
    },
    /// Create a guild sticker (requires file upload)
    Create {
        #[arg(long)]
        guild_id: String,
        /// Sticker name
        #[arg(long)]
        name: String,
        /// Sticker description
        #[arg(long)]
        description: String,
        /// Autocomplete/suggestion tags (comma separated)
        #[arg(long)]
        tags: String,
        /// Path to the sticker image file
        #[arg(long)]
        file: String,
    },
    /// Edit a guild sticker
    Edit {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
    },
    /// Delete a guild sticker
    Delete {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        id: String,
    },
}

impl StickerCommand {
    pub async fn execute(self, client: &DiscordClient, output_format: &OutputFormat) -> Result<()> {
        match self {
            Self::List { guild_id } => {
                let stickers = sticker::list_guild_stickers(client, &guild_id).await?;
                output::render_list(output_format, &stickers)?;
            }
            Self::Get { guild_id, id } => {
                let s = sticker::get_guild_sticker(client, &guild_id, &id).await?;
                output::render(output_format, &s)?;
            }
            Self::Create {
                guild_id,
                name,
                description,
                tags,
                file,
            } => {
                let s = sticker::create_guild_sticker(
                    client,
                    &guild_id,
                    &name,
                    &description,
                    &tags,
                    &file,
                )
                .await?;
                output::render(output_format, &s)?;
            }
            Self::Edit {
                guild_id,
                id,
                name,
                description,
            } => {
                let mut params = serde_json::Map::new();
                if let Some(name) = name {
                    params.insert("name".to_string(), serde_json::Value::String(name));
                }
                if let Some(desc) = description {
                    params.insert("description".to_string(), serde_json::Value::String(desc));
                }
                let s = sticker::edit_guild_sticker(
                    client,
                    &guild_id,
                    &id,
                    &serde_json::Value::Object(params),
                )
                .await?;
                output::render(output_format, &s)?;
            }
            Self::Delete { guild_id, id } => {
                sticker::delete_guild_sticker(client, &guild_id, &id).await?;
                println!("Sticker deleted successfully.");
            }
        }
        Ok(())
    }
}
