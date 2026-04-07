use anyhow::Result;
use clap::Subcommand;

use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum InviteCommand {
    /// Get an invite by code
    Get {
        /// Invite code
        #[arg(long)]
        code: String,
    },
    /// List guild invites
    List {
        /// Guild ID
        #[arg(long)]
        guild_id: String,
    },
    /// Create a channel invite
    Create {
        /// Channel ID
        #[arg(long)]
        channel_id: String,
        /// Max age in seconds (0 for never)
        #[arg(long, default_value = "86400")]
        max_age: u64,
        /// Max number of uses (0 for unlimited)
        #[arg(long, default_value = "0")]
        max_uses: u64,
        /// Whether this invite is temporary
        #[arg(long, default_value = "false")]
        temporary: bool,
    },
    /// Delete an invite
    Delete {
        /// Invite code
        #[arg(long)]
        code: String,
    },
}

pub async fn execute(
    client: &DiscordClient,
    command: InviteCommand,
    output_format: &OutputFormat,
) -> Result<()> {
    match command {
        InviteCommand::Get { code } => {
            let invite = client.get_invite(&code).await?;
            output::render(output_format, &invite)?;
        }
        InviteCommand::List { guild_id } => {
            let invites = client.list_guild_invites(&guild_id).await?;
            output::render_list(output_format, &invites)?;
        }
        InviteCommand::Create {
            channel_id,
            max_age,
            max_uses,
            temporary,
        } => {
            let params = serde_json::json!({
                "max_age": max_age,
                "max_uses": max_uses,
                "temporary": temporary,
            });
            let invite = client.create_channel_invite(&channel_id, &params).await?;
            output::render(output_format, &invite)?;
        }
        InviteCommand::Delete { code } => {
            client.delete_invite(&code).await?;
            println!("Invite {} deleted.", code);
        }
    }
    Ok(())
}
