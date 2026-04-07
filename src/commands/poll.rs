use anyhow::Result;
use clap::Subcommand;

use crate::api::poll;
use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum PollCommand {
    /// Get voters for a poll answer
    Voters {
        #[arg(long)]
        channel_id: String,
        #[arg(long)]
        message_id: String,
        #[arg(long)]
        answer_id: String,
        /// Pagination: get users after this user ID
        #[arg(long)]
        after: Option<String>,
        /// Maximum number of users to return (1-100)
        #[arg(long)]
        limit: Option<u64>,
    },
    /// Expire (end) a poll
    Expire {
        #[arg(long)]
        channel_id: String,
        #[arg(long)]
        message_id: String,
    },
}

impl PollCommand {
    pub async fn execute(self, client: &DiscordClient, output_format: &OutputFormat) -> Result<()> {
        match self {
            Self::Voters {
                channel_id,
                message_id,
                answer_id,
                after,
                limit,
            } => {
                let response = poll::get_answer_voters(
                    client,
                    &channel_id,
                    &message_id,
                    &answer_id,
                    after.as_deref(),
                    limit,
                )
                .await?;
                output::render_list(output_format, &response.users)?;
            }
            Self::Expire {
                channel_id,
                message_id,
            } => {
                let message = poll::expire_poll(client, &channel_id, &message_id).await?;
                let output_str = serde_json::to_string_pretty(&message)?;
                println!("{}", output_str);
            }
        }
        Ok(())
    }
}
