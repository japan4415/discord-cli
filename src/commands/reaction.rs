use anyhow::Result;
use clap::Subcommand;

use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum ReactionCommand {
    /// Add a reaction to a message
    Add {
        #[arg(long)]
        channel_id: String,
        #[arg(long)]
        message_id: String,
        /// Emoji (Unicode emoji or name:id for custom)
        #[arg(long)]
        emoji: String,
    },
    /// Remove your own reaction
    Remove {
        #[arg(long)]
        channel_id: String,
        #[arg(long)]
        message_id: String,
        #[arg(long)]
        emoji: String,
    },
    /// List users who reacted with an emoji
    List {
        #[arg(long)]
        channel_id: String,
        #[arg(long)]
        message_id: String,
        #[arg(long)]
        emoji: String,
    },
    /// Clear all reactions from a message
    Clear {
        #[arg(long)]
        channel_id: String,
        #[arg(long)]
        message_id: String,
    },
    /// Clear reactions for a specific emoji
    ClearEmoji {
        #[arg(long)]
        channel_id: String,
        #[arg(long)]
        message_id: String,
        #[arg(long)]
        emoji: String,
    },
}

impl ReactionCommand {
    pub async fn execute(self, client: &DiscordClient, output_format: &OutputFormat) -> Result<()> {
        match self {
            Self::Add {
                channel_id,
                message_id,
                emoji,
            } => {
                client
                    .add_reaction(&channel_id, &message_id, &emoji)
                    .await?;
                println!("Reaction {} added.", emoji);
            }
            Self::Remove {
                channel_id,
                message_id,
                emoji,
            } => {
                client
                    .remove_own_reaction(&channel_id, &message_id, &emoji)
                    .await?;
                println!("Reaction {} removed.", emoji);
            }
            Self::List {
                channel_id,
                message_id,
                emoji,
            } => {
                let users = client
                    .list_reactions(&channel_id, &message_id, &emoji)
                    .await?;
                output::render_list(output_format, &users)?;
            }
            Self::Clear {
                channel_id,
                message_id,
            } => {
                client.clear_all_reactions(&channel_id, &message_id).await?;
                println!("All reactions cleared.");
            }
            Self::ClearEmoji {
                channel_id,
                message_id,
                emoji,
            } => {
                client
                    .clear_emoji_reactions(&channel_id, &message_id, &emoji)
                    .await?;
                println!("Reactions for {} cleared.", emoji);
            }
        }
        Ok(())
    }
}
