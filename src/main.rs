use clap::{Parser, Subcommand};

mod commands {
    pub mod config;
    pub mod r#gen;
    pub mod setup;
}
mod config;

#[derive(Parser)]
#[command(name = "coto", version = "0.1.0", author)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a boto3 snippet from natural language
    Gen {
        #[arg(short, long)]
        prompt: Option<String>,
        #[arg(short, long)]
        region: Option<String>,
        #[arg(short = 'P', long)]
        profile: Option<String>,
        #[arg(short, long, default_value = "gpt-3.5-turbo")]
        model: String,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(long)]
        dry_run: bool,
    },

    /// Manage coto configuration
    Config {
        #[command(subcommand)]
        action: commands::config::ConfigAction,
    },

    /// Interactive first-time setup
    Setup,
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Set a config key (openai_key, default_profile, default_region, model)
    Set { key: String, value: String },
    /// Show current config
    Show,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::Gen {
            prompt,
            region,
            profile,
            model,
            output,
            dry_run,
        } => commands::r#gen::run(prompt, region, profile, model, output, dry_run),
        Commands::Config { action } => commands::config::run(action),
        Commands::Setup => commands::setup::run(),
    }
}
