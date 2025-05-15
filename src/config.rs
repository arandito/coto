use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{env, fs};

/// Represents the contents of ~/.coto/config.toml
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    /// OpenAI API key
    pub openai_key: Option<String>,
    /// Default AWS CLI profile name for snippet generation
    pub default_profile: Option<String>,
    /// Default AWS region for snippet generation
    pub default_region: Option<String>,
    /// Default LLM model
    pub model: Option<String>,
}

impl Config {
    /// Path to the config file (~/.coto/config.toml)
    fn path() -> Result<PathBuf> {
        // Determine base config dir: XDG_CONFIG_HOME or ~/.config
        let base = if let Some(dir) = env::var_os("XDG_CONFIG_HOME") {
            PathBuf::from(dir)
        } else {
            dirs::home_dir()
                .context("Could not determine home directory")?
                .join(".config")
        };
        let mut dir = base.join("coto");
        fs::create_dir_all(&dir).context("Failed to create config directory")?;
        dir.push("config.toml");
        Ok(dir)
    }

    /// Load config from file, returning defaults if not present
    pub fn load() -> Result<Self> {
        let path = Self::path()?;
        if !path.exists() {
            return Ok(Config::default());
        }
        let contents = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        let cfg: Config = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;
        Ok(cfg)
    }

    /// Save the config back to file (overwrites existing)
    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        let toml_str =
            toml::to_string_pretty(&self).context("Failed to serialize config to TOML")?;
        fs::write(&path, toml_str)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;
        Ok(())
    }
}
