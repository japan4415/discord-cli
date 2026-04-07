pub mod audit_log;
pub mod auto_moderation;
pub mod ban;
pub mod channel;
pub mod command;
pub mod emoji;
pub mod gateway;
pub mod guild;
pub mod invite;
pub mod member;
pub mod message;
pub mod pin;
pub mod poll;
pub mod reaction;
pub mod role;
pub mod scheduled_event;
pub mod soundboard;
pub mod stage;
pub mod sticker;
pub mod template;
pub mod thread;
pub mod user;
pub mod voice;
pub mod webhook;

use anyhow::Result;

use crate::cli::SubCommand;
use crate::client::DiscordClient;
use crate::output::OutputFormat;

pub async fn execute(
    client: &DiscordClient,
    command: SubCommand,
    output: &OutputFormat,
) -> Result<()> {
    match command {
        SubCommand::Guild(cmd) => cmd.execute(client, output).await,
        SubCommand::Channel(cmd) => cmd.execute(client, output).await,
        SubCommand::Message(cmd) => cmd.execute(client, output).await,
        SubCommand::User(cmd) => cmd.execute(client, output).await,
        SubCommand::Member(cmd) => cmd.execute(client, output).await,
        SubCommand::Role(cmd) => cmd.execute(client, output).await,
        SubCommand::Ban(cmd) => cmd.execute(client, output).await,
        SubCommand::Reaction(cmd) => cmd.execute(client, output).await,
        SubCommand::Emoji(cmd) => emoji::execute(client, cmd, output).await,
        SubCommand::Webhook(cmd) => webhook::execute(client, cmd, output).await,
        SubCommand::Invite(cmd) => invite::execute(client, cmd, output).await,
        SubCommand::AuditLog(cmd) => audit_log::execute(client, cmd, output).await,
        SubCommand::Command(cmd) => command::execute(client, cmd, output).await,
        SubCommand::Thread(cmd) => thread::execute(client, cmd, output).await,
        SubCommand::Pin(cmd) => pin::execute(client, cmd, output).await,
        SubCommand::Sticker(cmd) => cmd.execute(client, output).await,
        SubCommand::Stage(cmd) => cmd.execute(client, output).await,
        SubCommand::ScheduledEvent(cmd) => cmd.execute(client, output).await,
        SubCommand::AutoMod(cmd) => cmd.execute(client, output).await,
        SubCommand::Voice(cmd) => cmd.execute(client, output).await,
        SubCommand::Template(cmd) => cmd.execute(client, output).await,
        SubCommand::Poll(cmd) => cmd.execute(client, output).await,
        SubCommand::Soundboard(cmd) => cmd.execute(client, output).await,
        SubCommand::Gateway(cmd) => cmd.execute(client, output).await,
    }
}
