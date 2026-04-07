use anyhow::Result;
use clap::Subcommand;

use crate::client::DiscordClient;
use crate::output::{self, OutputFormat};

#[derive(Debug, Clone, Subcommand)]
pub enum RoleCommand {
    /// List roles in a guild
    List {
        #[arg(long)]
        guild_id: String,
    },
    /// Create a role
    Create {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        name: String,
        #[arg(long)]
        color: Option<u32>,
        #[arg(long)]
        hoist: Option<bool>,
        #[arg(long)]
        mentionable: Option<bool>,
    },
    /// Edit a role
    Edit {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        color: Option<u32>,
    },
    /// Delete a role
    Delete {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        id: String,
    },
    /// Assign a role to a member
    Assign {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        user_id: String,
        #[arg(long)]
        role_id: String,
    },
    /// Remove a role from a member
    Unassign {
        #[arg(long)]
        guild_id: String,
        #[arg(long)]
        user_id: String,
        #[arg(long)]
        role_id: String,
    },
}

impl RoleCommand {
    pub async fn execute(self, client: &DiscordClient, output_format: &OutputFormat) -> Result<()> {
        match self {
            Self::List { guild_id } => {
                let roles = client.list_roles(&guild_id).await?;
                output::render_list(output_format, &roles)?;
            }
            Self::Create {
                guild_id,
                name,
                color,
                hoist,
                mentionable,
            } => {
                let mut params = serde_json::Map::new();
                params.insert("name".into(), serde_json::Value::String(name));
                if let Some(c) = color {
                    params.insert("color".into(), serde_json::json!(c));
                }
                if let Some(h) = hoist {
                    params.insert("hoist".into(), serde_json::json!(h));
                }
                if let Some(m) = mentionable {
                    params.insert("mentionable".into(), serde_json::json!(m));
                }
                let role = client
                    .create_role(&guild_id, &serde_json::Value::Object(params))
                    .await?;
                output::render(output_format, &role)?;
            }
            Self::Edit {
                guild_id,
                id,
                name,
                color,
            } => {
                let mut params = serde_json::Map::new();
                if let Some(n) = name {
                    params.insert("name".into(), serde_json::Value::String(n));
                }
                if let Some(c) = color {
                    params.insert("color".into(), serde_json::json!(c));
                }
                let role = client
                    .edit_role(&guild_id, &id, &serde_json::Value::Object(params))
                    .await?;
                output::render(output_format, &role)?;
            }
            Self::Delete { guild_id, id } => {
                client.delete_role(&guild_id, &id).await?;
                println!("Role {} deleted.", id);
            }
            Self::Assign {
                guild_id,
                user_id,
                role_id,
            } => {
                client
                    .add_role_to_member(&guild_id, &user_id, &role_id)
                    .await?;
                println!("Role {} assigned to user {}.", role_id, user_id);
            }
            Self::Unassign {
                guild_id,
                user_id,
                role_id,
            } => {
                client
                    .remove_role_from_member(&guild_id, &user_id, &role_id)
                    .await?;
                println!("Role {} removed from user {}.", role_id, user_id);
            }
        }
        Ok(())
    }
}
