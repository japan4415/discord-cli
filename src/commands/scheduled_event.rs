use anyhow::Result;
use clap::Subcommand;

use crate::api::scheduled_event;
use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum ScheduledEventCommand {
    /// List scheduled events in a guild
    List {
        #[arg(long)]
        guild_id: String,
    },
    /// Get a specific scheduled event
    Get {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        id: String,
    },
    /// Create a scheduled event (JSON params)
    Create {
        #[arg(long)]
        guild_id: String,
        /// JSON data for event creation
        #[arg(long)]
        json: String,
    },
    /// Edit a scheduled event (JSON params)
    Edit {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        id: String,
        /// JSON data for event update
        #[arg(long)]
        json: String,
    },
    /// Delete a scheduled event
    Delete {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        id: String,
    },
    /// List users interested in a scheduled event
    Users {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        id: String,
    },
}

impl ScheduledEventCommand {
    pub async fn execute(self, client: &DiscordClient, output_format: &OutputFormat) -> Result<()> {
        match self {
            Self::List { guild_id } => {
                let events = scheduled_event::list_events(client, &guild_id).await?;
                output::render_list(output_format, &events)?;
            }
            Self::Get { guild_id, id } => {
                let event = scheduled_event::get_event(client, &guild_id, &id).await?;
                output::render(output_format, &event)?;
            }
            Self::Create { guild_id, json } => {
                let params: serde_json::Value = serde_json::from_str(&json)?;
                let event = scheduled_event::create_event(client, &guild_id, &params).await?;
                output::render(output_format, &event)?;
            }
            Self::Edit { guild_id, id, json } => {
                let params: serde_json::Value = serde_json::from_str(&json)?;
                let event = scheduled_event::edit_event(client, &guild_id, &id, &params).await?;
                output::render(output_format, &event)?;
            }
            Self::Delete { guild_id, id } => {
                scheduled_event::delete_event(client, &guild_id, &id).await?;
                println!("Scheduled event deleted successfully.");
            }
            Self::Users { guild_id, id } => {
                let users = scheduled_event::list_event_users(client, &guild_id, &id).await?;
                output::render_list(output_format, &users)?;
            }
        }
        Ok(())
    }
}
