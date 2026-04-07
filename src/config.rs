use anyhow::{bail, Result};

pub fn resolve_token(cli_token: Option<&str>) -> Result<String> {
    if let Some(token) = cli_token {
        return Ok(token.to_string());
    }

    if let Ok(token) = std::env::var("DISCORD_TOKEN") {
        if !token.is_empty() {
            return Ok(token);
        }
    }

    bail!("Discord token not provided. Use --token or set DISCORD_TOKEN environment variable.")
}
