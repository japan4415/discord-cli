use anyhow::Result;
use clap::Subcommand;

use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum ThreadCommand {
    /// Create a new thread
    Create {
        /// Channel ID
        channel_id: String,
        /// Thread name
        name: String,
        /// Thread type: public or private
        #[arg(long, default_value = "public")]
        r#type: String,
    },
    /// Join a thread
    Join {
        /// Thread ID
        id: String,
    },
    /// Leave a thread
    Leave {
        /// Thread ID
        id: String,
    },
    /// List thread members
    Members {
        /// Thread ID
        id: String,
    },
    /// List active threads in a guild
    ListActive {
        /// Guild ID
        guild_id: String,
    },
    /// List archived threads in a channel
    ListArchived {
        /// Channel ID
        channel_id: String,
        /// Archive type: public or private
        #[arg(long, default_value = "public")]
        r#type: String,
    },
}

pub async fn execute(
    client: &DiscordClient,
    command: ThreadCommand,
    output_format: &OutputFormat,
) -> Result<()> {
    match command {
        ThreadCommand::Create {
            channel_id,
            name,
            r#type,
        } => {
            let channel_type = match r#type.as_str() {
                "private" => 12,
                _ => 11, // public
            };
            let params = serde_json::json!({
                "name": name,
                "type": channel_type,
            });
            let thread = client.create_thread(&channel_id, &params).await?;
            output::render(output_format, &thread)?;
        }
        ThreadCommand::Join { id } => {
            client.join_thread(&id).await?;
            println!("Joined thread {}.", id);
        }
        ThreadCommand::Leave { id } => {
            client.leave_thread(&id).await?;
            println!("Left thread {}.", id);
        }
        ThreadCommand::Members { id } => {
            let members = client.list_thread_members(&id).await?;
            output::render_list(output_format, &members)?;
        }
        ThreadCommand::ListActive { guild_id } => {
            let response = client.list_active_threads(&guild_id).await?;
            output::render_list(output_format, &response.threads)?;
        }
        ThreadCommand::ListArchived { channel_id, r#type } => {
            let response = client.list_archived_threads(&channel_id, &r#type).await?;
            output::render_list(output_format, &response.threads)?;
        }
    }
    Ok(())
}
