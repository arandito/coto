use crate::config::Config;
use anyhow::{Context, Result};
use clap::Subcommand;
use toml;

/// Explicit, guided config subcommands
#[derive(Subcommand)]
pub enum ConfigAction {
    /// Store OpenAI API key
    #[command(name = "openai-key")]
    OpenaiKey { key: String },
    /// Store default AWS CLI profile
    #[command(name = "default-profile")]
    DefaultProfile { profile: String },
    /// Store default AWS region
    #[command(name = "default-region")]
    DefaultRegion { region: String },
    /// Store default LLM model (e.g. o4-mini)
    #[command(name = "model")]
    Model { model: String },
    /// Show current config
    #[command(name = "show")]
    Show,
}

/// Handle the `coto config` subcommands
pub fn run(action: ConfigAction) -> Result<()> {
    // Load existing config (or defaults)
    let mut cfg = Config::load()?;

    match action {
        ConfigAction::OpenaiKey { key } => {
            cfg.openai_key = Some(key);
            println!("✅ OpenAI API key stored.");
        }
        ConfigAction::DefaultProfile { profile } => {
            cfg.default_profile = Some(profile);
            println!("✅ Default AWS profile stored.");
        }
        ConfigAction::DefaultRegion { region } => {
            cfg.default_region = Some(region);
            println!("✅ Default AWS region stored.");
        }
        ConfigAction::Model { model } => {
            cfg.model = Some(model);
            println!("✅ Default LLM model stored.");
        }
        ConfigAction::Show => {
            let toml_str =
                toml::to_string_pretty(&cfg).context("Failed to serialize config for display")?;
            println!("{}", toml_str);
            return Ok(());
        }
    }
    cfg.save()?;
    Ok(())
}
