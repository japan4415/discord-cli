use anyhow::Result;
use clap::Subcommand;

use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum UserCommand {
    /// Get a user by ID
    Get {
        #[arg(long)]
        id: String,
    },
    /// Get the current authenticated user
    Me,
}

impl UserCommand {
    pub async fn execute(self, client: &DiscordClient, output_format: &OutputFormat) -> Result<()> {
        match self {
            Self::Get { id } => {
                let user = client.get_user(&id).await?;
                output::render(output_format, &user)?;
            }
            Self::Me => {
                let user = client.get_current_user().await?;
                output::render(output_format, &user)?;
            }
        }
        Ok(())
    }
}
