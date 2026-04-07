use anyhow::Result;
use clap::Subcommand;

use crate::api::soundboard;
use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum SoundboardCommand {
    /// List default soundboard sounds
    ListDefault,
    /// List guild soundboard sounds
    List {
        #[arg(long)]
        guild_id: String,
    },
}

impl SoundboardCommand {
    pub async fn execute(self, client: &DiscordClient, output_format: &OutputFormat) -> Result<()> {
        match self {
            Self::ListDefault => {
                let sounds = soundboard::list_default_soundboard_sounds(client).await?;
                output::render_list(output_format, &sounds)?;
            }
            Self::List { guild_id } => {
                let sounds = soundboard::list_guild_soundboard_sounds(client, &guild_id).await?;
                output::render_list(output_format, &sounds)?;
            }
        }
        Ok(())
    }
}
