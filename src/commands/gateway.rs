use anyhow::Result;
use clap::Subcommand;

use crate::api::gateway;
use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum GatewayCommand {
    /// Get gateway URL
    Get,
    /// Get gateway bot info (includes shards and session start limit)
    Bot,
}

impl GatewayCommand {
    pub async fn execute(self, client: &DiscordClient, output_format: &OutputFormat) -> Result<()> {
        match self {
            Self::Get => {
                let info = gateway::get_gateway(client).await?;
                output::render(output_format, &info)?;
            }
            Self::Bot => {
                let info = gateway::get_gateway_bot(client).await?;
                output::render(output_format, &info)?;
            }
        }
        Ok(())
    }
}
