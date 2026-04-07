use anyhow::Result;
use clap::Subcommand;

use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum WebhookCommand {
    /// List webhooks for a channel
    List {
        /// Channel ID
        channel_id: String,
    },
    /// Get a webhook by ID
    Get {
        /// Webhook ID
        id: String,
    },
    /// Create a webhook
    Create {
        /// Channel ID
        channel_id: String,
        /// Webhook name
        name: String,
    },
    /// Edit a webhook
    Edit {
        /// Webhook ID
        id: String,
        /// New name
        name: String,
    },
    /// Delete a webhook
    Delete {
        /// Webhook ID
        id: String,
    },
    /// Execute a webhook
    Execute {
        /// Webhook ID
        id: String,
        /// Webhook token
        token: String,
        /// Message content
        content: String,
    },
}

pub async fn execute(
    client: &DiscordClient,
    command: WebhookCommand,
    output_format: &OutputFormat,
) -> Result<()> {
    match command {
        WebhookCommand::List { channel_id } => {
            let webhooks = client.list_channel_webhooks(&channel_id).await?;
            output::render_list(output_format, &webhooks)?;
        }
        WebhookCommand::Get { id } => {
            let webhook = client.get_webhook(&id).await?;
            output::render(output_format, &webhook)?;
        }
        WebhookCommand::Create { channel_id, name } => {
            let params = serde_json::json!({ "name": name });
            let webhook = client.create_webhook(&channel_id, &params).await?;
            output::render(output_format, &webhook)?;
        }
        WebhookCommand::Edit { id, name } => {
            let params = serde_json::json!({ "name": name });
            let webhook = client.edit_webhook(&id, &params).await?;
            output::render(output_format, &webhook)?;
        }
        WebhookCommand::Delete { id } => {
            client.delete_webhook(&id).await?;
            println!("Webhook {} deleted.", id);
        }
        WebhookCommand::Execute { id, token, content } => {
            let params = serde_json::json!({ "content": content });
            client.execute_webhook(&id, &token, &params).await?;
            println!("Webhook executed.");
        }
    }
    Ok(())
}
