use anyhow::Result;
use clap::Subcommand;

use crate::client::DiscordClient;
use crate::models::common::ImageData;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum EmojiCommand {
    /// List all emojis in a guild
    List {
        /// Guild ID
        guild_id: String,
    },
    /// Get a specific emoji
    Get {
        /// Guild ID
        guild_id: String,
        /// Emoji ID
        id: String,
    },
    /// Create a new emoji
    Create {
        /// Guild ID
        guild_id: String,
        /// Emoji name
        name: String,
        /// Path to image file
        image_path: String,
    },
    /// Edit an emoji
    Edit {
        /// Guild ID
        guild_id: String,
        /// Emoji ID
        id: String,
        /// New name
        name: String,
    },
    /// Delete an emoji
    Delete {
        /// Guild ID
        guild_id: String,
        /// Emoji ID
        id: String,
    },
}

pub async fn execute(
    client: &DiscordClient,
    command: EmojiCommand,
    output_format: &OutputFormat,
) -> Result<()> {
    match command {
        EmojiCommand::List { guild_id } => {
            let emojis = client.list_guild_emojis(&guild_id).await?;
            output::render_list(output_format, &emojis)?;
        }
        EmojiCommand::Get { guild_id, id } => {
            let emoji = client.get_guild_emoji(&guild_id, &id).await?;
            output::render(output_format, &emoji)?;
        }
        EmojiCommand::Create {
            guild_id,
            name,
            image_path,
        } => {
            let bytes = std::fs::read(&image_path)?;
            let content_type = if image_path.ends_with(".png") {
                "image/png"
            } else if image_path.ends_with(".gif") {
                "image/gif"
            } else {
                "image/jpeg"
            };
            let image_data = ImageData::from_bytes(&bytes, content_type);
            let params = serde_json::json!({
                "name": name,
                "image": image_data.0,
            });
            let emoji = client.create_guild_emoji(&guild_id, &params).await?;
            output::render(output_format, &emoji)?;
        }
        EmojiCommand::Edit { guild_id, id, name } => {
            let params = serde_json::json!({ "name": name });
            let emoji = client.edit_guild_emoji(&guild_id, &id, &params).await?;
            output::render(output_format, &emoji)?;
        }
        EmojiCommand::Delete { guild_id, id } => {
            client.delete_guild_emoji(&guild_id, &id).await?;
            println!("Emoji {} deleted.", id);
        }
    }
    Ok(())
}
