use anyhow::Result;

/// Stub for the `gen` command -- will implement OpenAI call & templating next

pub fn run(
    prompt: Option<String>,
    region: Option<String>,
    profile: Option<String>,
    model: String,
    output: Option<String>,
    dry_run: bool,
) -> Result<()> {
    unimplemented!("gen command not implemented yet");
}
