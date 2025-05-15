use crate::config::Config;
use anyhow::Result;
use dialoguer::{Confirm, Input};
use toml;

/// Interactive setup: guide the user through intiial config
pub fn run() -> Result<()> {
    // Display a friendly header
    println!("============================");
    println!(" Coto Initial Setup Wizard ");
    println!("============================\n");
    // Load existing or default config
    let mut cfg = Config::load()?;
    // OpenAI key
    let key: String = Input::new()
        .with_prompt("Enter your OpenAI API key")
        .allow_empty(false)
        .interact_text()?;
    cfg.openai_key = Some(key);
    // AWS profile
    if Confirm::new()
        .with_prompt("Set a default AWS CLI profile?")
        .default(true)
        .interact()?
    {
        let default_profile = cfg
            .default_profile
            .clone()
            .unwrap_or_else(|| "default".into());
        let profile: String = Input::new()
            .with_prompt("AWS profile name (as in ~/.aws/credentials or ~/.aws/config)")
            .default(default_profile)
            .interact_text()?;
        cfg.default_profile = Some(profile);
    }
    // AWS region
    if Confirm::new()
        .with_prompt("Set a default AWS region?")
        .default(true)
        .interact()?
    {
        let default_region = cfg
            .default_region
            .clone()
            .unwrap_or_else(|| "us-east-1".into());
        let region: String = Input::new()
            .with_prompt("AWS region")
            .default(default_region)
            .interact_text()?;
        cfg.default_region = Some(region);
    }
    // LLM model
    let default_model = cfg.model.clone().unwrap_or_else(|| "o4-mini".into());
    let model: String = Input::new()
        .with_prompt("Default LLM model")
        .default(default_model)
        .interact_text()?;
    cfg.model = Some(model);
    // Confirm & save
    print!(
        "\nConfiguration to be saved:\n{}",
        toml::to_string_pretty(&cfg)?,
    );
    if Confirm::new()
        .with_prompt("Save to ~/.config/config.toml?")
        .default(true)
        .interact()?
    {
        cfg.save()?;
        println!("✅ Configuration saved.");
    } else {
        println!("⚠️  Setup aborted — no changes written.");
    }
    Ok(())
}
