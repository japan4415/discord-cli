use anyhow::Result;
use clap::Subcommand;

use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum MessageCommand {
    /// Get a message
    Get {
        #[arg(long)]
        channel_id: String,
        #[arg(long)]
        id: String,
    },
    /// List messages in a channel
    List {
        #[arg(long)]
        channel_id: String,
        #[arg(long, default_value = "50")]
        limit: u64,
        #[arg(long)]
        before: Option<String>,
        #[arg(long)]
        after: Option<String>,
    },
    /// Send a message to a channel
    Send {
        #[arg(long)]
        channel_id: String,
        #[arg(long)]
        content: String,
    },
    /// Edit a message
    Edit {
        #[arg(long)]
        channel_id: String,
        #[arg(long)]
        id: String,
        #[arg(long)]
        content: String,
    },
    /// Delete a message
    Delete {
        #[arg(long)]
        channel_id: String,
        #[arg(long)]
        id: String,
    },
    /// Bulk delete messages (2-100 messages, max 14 days old)
    BulkDelete {
        #[arg(long)]
        channel_id: String,
        /// Comma-separated message IDs
        #[arg(long, value_delimiter = ',')]
        ids: Vec<String>,
    },
    /// Crosspost a message in an announcement channel
    Crosspost {
        #[arg(long)]
        channel_id: String,
        #[arg(long)]
        id: String,
    },
}

impl MessageCommand {
    pub async fn execute(self, client: &DiscordClient, output_format: &OutputFormat) -> Result<()> {
        match self {
            Self::Get { channel_id, id } => {
                let message = client.get_message(&channel_id, &id).await?;
                output::render(output_format, &message)?;
            }
            Self::List {
                channel_id,
                limit,
                before,
                after,
            } => {
                let messages = client
                    .list_messages(
                        &channel_id,
                        Some(limit),
                        before.as_deref(),
                        after.as_deref(),
                    )
                    .await?;
                output::render_list(output_format, &messages)?;
            }
            Self::Send {
                channel_id,
                content,
            } => {
                let params = serde_json::json!({ "content": content });
                let message = client.send_message(&channel_id, &params).await?;
                output::render(output_format, &message)?;
            }
            Self::Edit {
                channel_id,
                id,
                content,
            } => {
                let params = serde_json::json!({ "content": content });
                let message = client.edit_message(&channel_id, &id, &params).await?;
                output::render(output_format, &message)?;
            }
            Self::Delete { channel_id, id } => {
                client.delete_message(&channel_id, &id).await?;
                println!("Message {} deleted.", id);
            }
            Self::BulkDelete { channel_id, ids } => {
                client.bulk_delete_messages(&channel_id, &ids).await?;
                println!("{} messages deleted.", ids.len());
            }
            Self::Crosspost { channel_id, id } => {
                let message = client.crosspost_message(&channel_id, &id).await?;
                output::render(output_format, &message)?;
            }
        }
        Ok(())
    }
}
