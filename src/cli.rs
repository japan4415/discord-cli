use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(
    name = "discord-cli",
    version,
    about = "A comprehensive Discord REST API CLI tool"
)]
pub struct Cli {
    /// Discord bot or user token (can also be set via DISCORD_TOKEN env var)
    #[arg(long, env = "DISCORD_TOKEN", global = true, hide_env_values = true)]
    pub token: Option<String>,

    /// Token type
    #[arg(long, default_value = "bot", global = true)]
    pub token_type: TokenTypeArg,

    /// Output format
    #[arg(long, short, default_value = "table", global = true)]
    pub output: OutputFormatArg,

    /// Enable verbose/debug output
    #[arg(long, short, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<SubCommand>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum TokenTypeArg {
    Bot,
    User,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormatArg {
    Json,
    Table,
    JsonPretty,
}

#[derive(Debug, Clone, clap::Subcommand)]
pub enum SubCommand {
    /// Manage guilds/servers
    #[command(subcommand)]
    Guild(crate::commands::guild::GuildCommand),
    /// Manage channels
    #[command(subcommand)]
    Channel(crate::commands::channel::ChannelCommand),
    /// Manage messages
    #[command(subcommand)]
    Message(crate::commands::message::MessageCommand),
    /// Manage users
    #[command(subcommand)]
    User(crate::commands::user::UserCommand),
    /// Manage guild members
    #[command(subcommand)]
    Member(crate::commands::member::MemberCommand),
    /// Manage roles
    #[command(subcommand)]
    Role(crate::commands::role::RoleCommand),
    /// Manage bans
    #[command(subcommand)]
    Ban(crate::commands::ban::BanCommand),
    /// Manage reactions
    #[command(subcommand)]
    Reaction(crate::commands::reaction::ReactionCommand),
    /// Manage guild emojis
    #[command(subcommand)]
    Emoji(crate::commands::emoji::EmojiCommand),
    /// Manage webhooks
    #[command(subcommand)]
    Webhook(crate::commands::webhook::WebhookCommand),
    /// Manage invites
    #[command(subcommand)]
    Invite(crate::commands::invite::InviteCommand),
    /// View audit logs
    #[command(subcommand)]
    AuditLog(crate::commands::audit_log::AuditLogCommand),
    /// Manage application commands
    #[command(subcommand)]
    Command(crate::commands::command::CommandCommand),
    /// Manage threads
    #[command(subcommand)]
    Thread(crate::commands::thread::ThreadCommand),
    /// Manage pinned messages
    #[command(subcommand)]
    Pin(crate::commands::pin::PinCommand),
    /// Manage guild stickers
    #[command(subcommand)]
    Sticker(crate::commands::sticker::StickerCommand),
    /// Manage stage instances
    #[command(subcommand)]
    Stage(crate::commands::stage::StageCommand),
    /// Manage guild scheduled events
    #[command(subcommand)]
    ScheduledEvent(crate::commands::scheduled_event::ScheduledEventCommand),
    /// Manage auto-moderation rules
    #[command(subcommand)]
    AutoMod(crate::commands::auto_moderation::AutoModCommand),
    /// Voice region information
    #[command(subcommand)]
    Voice(crate::commands::voice::VoiceCommand),
    /// Manage guild templates
    #[command(subcommand)]
    Template(crate::commands::template::TemplateCommand),
    /// Poll operations
    #[command(subcommand)]
    Poll(crate::commands::poll::PollCommand),
    /// Soundboard sounds
    #[command(subcommand)]
    Soundboard(crate::commands::soundboard::SoundboardCommand),
    /// Gateway information
    #[command(subcommand)]
    Gateway(crate::commands::gateway::GatewayCommand),
}
