use anyhow::Result;
use clap::Subcommand;

use crate::api::voice;
use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum VoiceCommand {
    /// List available voice regions
    Regions,
}

impl VoiceCommand {
    pub async fn execute(self, client: &DiscordClient, output_format: &OutputFormat) -> Result<()> {
        match self {
            Self::Regions => {
                let regions = voice::list_voice_regions(client).await?;
                output::render_list(output_format, &regions)?;
            }
        }
        Ok(())
    }
}
