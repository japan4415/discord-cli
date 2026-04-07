use anyhow::Result;
use clap::Subcommand;

use crate::api::audit_log::AuditLogQuery;
use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum AuditLogCommand {
    /// Get audit log entries for a guild
    Get {
        /// Guild ID
        guild_id: String,
        /// Filter by user ID
        #[arg(long)]
        user_id: Option<String>,
        /// Filter by action type
        #[arg(long)]
        action_type: Option<u64>,
        /// Maximum number of entries
        #[arg(long)]
        limit: Option<u64>,
    },
}

pub async fn execute(
    client: &DiscordClient,
    command: AuditLogCommand,
    output_format: &OutputFormat,
) -> Result<()> {
    match command {
        AuditLogCommand::Get {
            guild_id,
            user_id,
            action_type,
            limit,
        } => {
            let query = AuditLogQuery {
                user_id,
                action_type,
                before: None,
                after: None,
                limit,
            };
            let audit_log = client.get_guild_audit_log(&guild_id, &query).await?;
            output::render_list(output_format, &audit_log.audit_log_entries)?;
        }
    }
    Ok(())
}
