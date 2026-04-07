use anyhow::Result;
use clap::Subcommand;

use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum PinCommand {
    /// List pinned messages in a channel
    List {
        /// Channel ID
        channel_id: String,
    },
    /// Pin a message
    Add {
        /// Channel ID
        channel_id: String,
        /// Message ID
        message_id: String,
    },
    /// Unpin a message
    Remove {
        /// Channel ID
        channel_id: String,
        /// Message ID
        message_id: String,
    },
}

pub async fn execute(
    client: &DiscordClient,
    command: PinCommand,
    output_format: &OutputFormat,
) -> Result<()> {
    match command {
        PinCommand::List { channel_id } => {
            let pins = client.list_pins(&channel_id).await?;
            output::render_list(output_format, &pins)?;
        }
        PinCommand::Add {
            channel_id,
            message_id,
        } => {
            client.pin_message(&channel_id, &message_id).await?;
            println!("Message {} pinned.", message_id);
        }
        PinCommand::Remove {
            channel_id,
            message_id,
        } => {
            client.unpin_message(&channel_id, &message_id).await?;
            println!("Message {} unpinned.", message_id);
        }
    }
    Ok(())
}
