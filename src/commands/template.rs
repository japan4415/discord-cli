use anyhow::Result;
use clap::Subcommand;

use crate::api::template;
use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum TemplateCommand {
    /// List guild templates
    List {
        #[arg(long)]
        guild_id: String,
    },
    /// Get a guild template by code
    Get {
        #[arg(long)]
        code: String,
    },
    /// Create a guild template
    Create {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        name: String,
        #[arg(long)]
        description: Option<String>,
    },
    /// Sync a guild template
    Sync {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        code: String,
    },
    /// Edit a guild template
    Edit {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        code: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
    },
    /// Delete a guild template
    Delete {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        code: String,
    },
}

impl TemplateCommand {
    pub async fn execute(self, client: &DiscordClient, output_format: &OutputFormat) -> Result<()> {
        match self {
            Self::List { guild_id } => {
                let templates = template::list_guild_templates(client, &guild_id).await?;
                output::render_list(output_format, &templates)?;
            }
            Self::Get { code } => {
                let t = template::get_template(client, &code).await?;
                output::render(output_format, &t)?;
            }
            Self::Create {
                guild_id,
                name,
                description,
            } => {
                let mut params = serde_json::json!({ "name": name });
                if let Some(desc) = description {
                    params["description"] = serde_json::Value::String(desc);
                }
                let t = template::create_guild_template(client, &guild_id, &params).await?;
                output::render(output_format, &t)?;
            }
            Self::Sync { guild_id, code } => {
                let t = template::sync_guild_template(client, &guild_id, &code).await?;
                output::render(output_format, &t)?;
            }
            Self::Edit {
                guild_id,
                code,
                name,
                description,
            } => {
                let mut params = serde_json::Map::new();
                if let Some(name) = name {
                    params.insert("name".to_string(), serde_json::Value::String(name));
                }
                if let Some(desc) = description {
                    params.insert("description".to_string(), serde_json::Value::String(desc));
                }
                let t = template::edit_guild_template(
                    client,
                    &guild_id,
                    &code,
                    &serde_json::Value::Object(params),
                )
                .await?;
                output::render(output_format, &t)?;
            }
            Self::Delete { guild_id, code } => {
                template::delete_guild_template(client, &guild_id, &code).await?;
                println!("Template deleted successfully.");
            }
        }
        Ok(())
    }
}
