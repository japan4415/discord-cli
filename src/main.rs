mod api;
mod cli;
mod client;
mod commands;
mod config;
mod error;
mod models;
mod output;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

use cli::Cli;
use client::{DiscordClient, TokenType};
use output::OutputFormat;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    let filter = if cli.verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::from_default_env()
    };
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let Some(command) = cli.command else {
        eprintln!("Error: A subcommand is required. Run with --help for usage information.");
        std::process::exit(1);
    };

    let token = config::resolve_token(cli.token.as_deref())?;
    let token_type = TokenType::from(cli.token_type);
    let output_format = OutputFormat::from(cli.output);

    let client = DiscordClient::new(token, token_type)?;
    commands::execute(&client, command, &output_format).await?;

    Ok(())
}
