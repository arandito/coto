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
        #[arg(short, long, help = "Generation prompt (required)")]
        prompt: Option<String>,
        #[arg(short, long, help = "AWS region (from config if unset)")]
        region: Option<String>,
        #[arg(short = 'P', long, help = "AWS profile (from config if unset)")]
        profile: Option<String>,
        #[arg(short, long, help = "LLM model (from config if unset)")]
        model: Option<String>,
        #[arg(short, long, help = "Output file path (prints to stdout if unset)")]
        output: Option<String>,
        #[arg(short, long, help = "Dry run to verify OpenAI request before send")]
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
