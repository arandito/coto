use crate::config::Config;
use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::{Value, json};
use std::{env, fs::File, io::Write};

/// Handle the `coto gen` subcommand: invoke OpenAI and generate a boto3 snippet
pub fn run(
    prompt: Option<String>,
    region: Option<String>,
    profile: Option<String>,
    model: Option<String>,
    output: Option<String>,
    dry_run: bool,
) -> Result<()> {
    // 1. Ensure prompt is provided
    let prompt = prompt.unwrap_or_else(|| {
        eprintln!("Error: --prompt is required for gen");
        std::process::exit(1);
    });
    // 2. Load config
    let cfg = Config::load()?;
    // 3. Resolve OpenAI API key
    let resolved_openai_key = cfg
        .openai_key
        .clone()
        .or_else(|| env::var("OPENAI_API_KEY").ok())
        .context(
            "OpenAI API key not set; run `coto config openai-key <KEY>` or set OPENAI_API_KEY",
        )?;
    // 4. Resolve model
    let resolved_model = model.or(cfg.model.clone());
    // 5. Resolve profile and region
    let resolved_profile = profile.or(cfg.default_profile.clone());
    let resolved_region = region.or(cfg.default_region.clone());
    // 6. Build system prompt with structured JSON schema instructions
    let mut system_prompt =
        String::from("You are an expert Python and Boto3 code generation assistant. ");
    system_prompt.push_str(
        "Given a user request for a boto3 Python snippet, you must respond with valid JSON only, following this schema: `{'code': '<python_code_snippet>'}`. "
    );
    system_prompt
        .push_str("Do NOT include any explanations or extra keys. Only output the JSON object.");
    // Add context for AWS profile/region
    if let Some(p) = &resolved_profile {
        system_prompt.push_str(&format!(" Use AWS CLI profile '{p}'.", p = p));
    }
    if let Some(r) = &resolved_region {
        system_prompt.push_str(&format!(" Set region_name to '{r}'.", r = r));
    }
    // 7. Construct request payload
    let input = vec![
        json!({"role": "system", "content": system_prompt}),
        json!({"role": "user",   "content": prompt}),
    ];
    let request_body = json!({
        "model": resolved_model,
        "input": input,
    });
    // 8. Dry-run support
    if dry_run {
        println!("Request payload (dry-run):");
        println!("{}", serde_json::to_string_pretty(&request_body)?);
        return Ok(());
    }
    // 9. Send request to OpenAI
    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/responses")
        .bearer_auth(resolved_openai_key)
        .json(&request_body)
        .send()
        .context("Failed to send request to OpenAI")?
        .error_for_status()
        .context("OpenAI API returned an error status")?;
    // 10. Parse and extract text content
    let resp_json: Value = response
        .json()
        .context("Failed to parse OpenAI response JSON")?;
    let content = resp_json["output"][0]["content"][0]["text"]
        .as_str()
        .context("No content in OpenAI response")?;
    // 11. Parse text content as JSON to enforce schema
    let parsed: Value = serde_json::from_str(content).context(
        "Failed to parse LLM response as JSON. Ensure the model follows the output schema.",
    )?;
    let code = parsed["code"]
        .as_str()
        .context("No 'code' key found in LLM response JSON")?;
    // 12. Output code to file or stdout
    if let Some(path) = output {
        let mut file = File::create(&path).context("Failed to create output file")?;
        file.write_all(code.as_bytes())
            .context("Failed to write snippet to file")?;
        println!("✅ Snippet written to {path}", path = path);
    } else {
        println!("{}", code);
    }
    Ok(())
}
